use core::fmt;
use std::{
    error::Error,
    fmt::Debug,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread, usize,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Threadpool {
    threads: Vec<Worker>,
    sender: Sender<Job>,
}

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<Arc<Mutex<Receiver<Job>>>>,
}

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Size must be greater than zero")
    }
}

impl Error for PoolCreationError {}

impl Threadpool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    pub fn build(size: usize) -> Result<Threadpool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError);
        }
        let (tx, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));
        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            threads.push(Worker::new(size, Arc::clone(&receiver)));
        }

        Ok(Threadpool {
            threads,
            sender: tx,
        })
    }
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let threads = thread::spawn(move || loop {
            let receiver = rx.lock().unwrap().recv().unwrap();
            print!("Worker {id} got a job; Executing Job");
            receiver();
        });
        Worker {
            id,
            thread: threads,
        }
    }
}
