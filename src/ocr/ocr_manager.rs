use std::thread::{Thread, JoinHandle, self};
use crate::ocr::ocr_task::OCRTask;
use crate::area::AreaTask;
use std::iter;
use std::sync::{Arc, Mutex};
use crate::error::KanjitomoError;
use std::sync::atomic::{AtomicBool, Ordering, AtomicU32};
use rayon::{ThreadPool, ThreadPoolBuilder};
use crossbeam::channel::{Sender, Receiver, unbounded};
use crate::util::{sharpen_image, make_bw};

const OCR_THREADS: usize = 8;

#[derive(Debug)]
pub(crate) struct OCRManager {
    thread_pool: ThreadPool,
    pending_tasks: Sender<OCRTask>,
    results: Receiver<OCRTask>,
    task_count: AtomicU32,
    stop_flag: Arc<AtomicBool>
}

impl OCRManager {
    pub(crate) fn new() -> Self {
        let mut thread_pool = ThreadPoolBuilder::new()
            .num_threads(OCR_THREADS)
            .thread_name(|idx| {
                format!("OCRTask ({})", idx)
            })
            .build()
            .unwrap();
        let (s, r) = unbounded();
        let (res_s, res_r) = unbounded();

        let stop_flag = Arc::new(AtomicBool::new(false));

        let mut mgr = Self {
            thread_pool,
            pending_tasks: s,
            results: res_r,
            task_count: Default::default(),
            stop_flag
        };
        mgr.install_threads(res_s, r);

        mgr
    }

    fn install_threads(&mut self, res_s: Sender<OCRTask>, r: Receiver<OCRTask>) {
        let stop_flag = self.stop_flag.clone();
        self.thread_pool.install(move || {
            for i in 0..OCR_THREADS {
                let res_s = res_s.clone();
                let r = r.clone();
                let stop_flag = stop_flag.clone();

                rayon::spawn(move || {
                    log::trace!("thread id is {:?} name is {:?}", thread::current().id(), thread::current().name());
                    loop {
                        if stop_flag.load(Ordering::Relaxed) {
                            log::trace!("{} got a shutdown request.", thread::current().name().unwrap());
                            break
                        }
                        if let Ok(mut task) = r.recv() {
                            log::trace!("got task in {:?}", thread::current().name());
                            let image = &task.image;
                            let sharpened = sharpen_image(image, None, None);
                            let b_w = make_bw(&sharpened, 135);
                            task.image = b_w;
                            res_s.send(task).unwrap();
                            log::trace!("Sent result to receiver from {:?}", thread::current().name());
                        }
                    }
                })
            }
        })
    }

    pub(crate) fn add_task(&mut self, task: OCRTask) {
        self.pending_tasks.send(task).unwrap();
        let mut task_count = self.task_count.load(Ordering::SeqCst);
        task_count += 1;
        self.task_count.store(task_count, Ordering::SeqCst);
    }

    pub(crate) fn stop_threads(&mut self) {
        self.stop_flag.store(true, Ordering::SeqCst)
    }

    pub(crate) fn wait_until_done(&mut self) {
        while self.task_count.load(Ordering::SeqCst) > 0 {
            if let Ok(task) = self.results.recv() {
                let mut task_count = self.task_count.load(Ordering::SeqCst);
                task_count -= 1;
                self.task_count.store(task_count, Ordering::SeqCst);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use image::{ImageBuffer, open};
    use crate::util::tests::PATH;

    #[test]
    fn threads_test() {
        pretty_env_logger::try_init().unwrap_or(());
        let mut mgr = Arc::new(Mutex::new(OCRManager::new()));
        {
            let mut mgr = mgr.clone();
            rayon::spawn(move || {
                for i in 0..3 {
                    let mut mgr = mgr.lock().unwrap();
                    mgr.add_task(OCRTask::new(ImageBuffer::new(32, 32)));
                }
                drop(mgr)
            })
        }

        let res = thread::spawn(move || {
            let mut mgr = mgr.lock().unwrap();
            mgr.wait_until_done();
            mgr.stop_threads();
        }).join().unwrap();
        log::trace!("Finished processing.");
        thread::sleep(Duration::from_micros(1000));
    }
}