use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    pub id: usize,
    pub thread: thread::JoinHandle<()>,
}
impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock()
                .expect("Could not lock")
                .recv()
                .expect("Could not get the Job");

            job();
        });

        Worker { id, thread }
    }
}
pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: mpsc::Sender<Job>,
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        if size <= 0 {
            panic!("The size of thread pool could not be zero or less");
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender
            .send(job)
            .expect("Could send the job through the sender");
    }
}
