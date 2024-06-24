use core::fmt;
use std::{
    error::Error,
    fmt::Debug,
    sync::{
        mpsc::{self, Sender},
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
    thread: Option<thread::JoinHandle<()>>,
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for Threadpool {
    fn drop(&mut self) {
        for worker in &mut self.threads {
            println!("TCP is shutting Down now");
            println!("Threads will stop receiving");
            println!("Shutting down Working {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
