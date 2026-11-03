//! One job at a time, on a thread of its own, and only ever the newest.
//!
//! What a window asks for as a person clicks around goes stale as soon as they click again. A job
//! that has not started when another is sent is forgotten, so there are never two of them reading
//! a failing drive at once, or loading its registry twice, however fast the clicks come.

use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread;

struct Waiting<J> {
    job: Option<J>,
    closed: bool,
}

struct Shared<J> {
    waiting: Mutex<Waiting<J>>,
    changed: Condvar,
}

impl<J> Shared<J> {
    /// A job that panicked is reported by whoever ran it, so what it left here is still good.
    fn lock(&self) -> MutexGuard<'_, Waiting<J>> {
        self.waiting
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

pub(crate) struct LatestWorker<J> {
    shared: Arc<Shared<J>>,
}

impl<J: Send + 'static> LatestWorker<J> {
    /// Start the thread. It runs each job with `run` and ends when this is dropped, after finishing
    /// any job it is running.
    pub(crate) fn new(mut run: impl FnMut(J) + Send + 'static) -> Self {
        let shared = Arc::new(Shared {
            waiting: Mutex::new(Waiting {
                job: None,
                closed: false,
            }),
            changed: Condvar::new(),
        });
        let on_thread = Arc::clone(&shared);
        thread::spawn(move || {
            loop {
                let job = {
                    let mut waiting = on_thread.lock();
                    loop {
                        if waiting.closed {
                            return;
                        }
                        if let Some(job) = waiting.job.take() {
                            break job;
                        }
                        waiting = on_thread
                            .changed
                            .wait(waiting)
                            .unwrap_or_else(std::sync::PoisonError::into_inner);
                    }
                };
                run(job);
            }
        });
        Self { shared }
    }

    /// Run this next, in place of anything that has not started.
    pub(crate) fn send(&self, job: J) {
        self.shared.lock().job = Some(job);
        self.shared.changed.notify_all();
    }
}

impl<J> Drop for LatestWorker<J> {
    fn drop(&mut self) {
        self.shared.lock().closed = true;
        self.shared.changed.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::time::Duration;

    use super::*;

    #[test]
    fn a_job_not_started_is_replaced_by_a_newer_one() {
        let (started, wait_for_start) = mpsc::channel();
        let (release, wait_for_release) = mpsc::channel::<()>();
        let (done, finished) = mpsc::channel();
        let worker = LatestWorker::new(move |job: u32| {
            let _ = started.send(job);
            let _ = wait_for_release.recv();
            let _ = done.send(job);
        });
        worker.send(1);
        assert_eq!(wait_for_start.recv_timeout(Duration::from_secs(30)), Ok(1));
        worker.send(2);
        worker.send(3);
        for _ in 0..2 {
            let _ = release.send(());
        }
        assert_eq!(finished.recv_timeout(Duration::from_secs(30)), Ok(1));
        assert_eq!(finished.recv_timeout(Duration::from_secs(30)), Ok(3));
        assert_eq!(wait_for_start.recv_timeout(Duration::from_secs(30)), Ok(3));
    }

    #[test]
    fn dropping_it_ends_the_thread() {
        let (done, gone) = mpsc::channel::<()>();
        let worker = LatestWorker::new(move |_: u32| {
            let _keep = &done;
        });
        drop(worker);
        // The closure, and the sender in it, go with the thread.
        assert_eq!(
            gone.recv_timeout(Duration::from_secs(30)),
            Err(mpsc::RecvTimeoutError::Disconnected)
        );
    }
}
