//! An HTTP web server providing a public API through the ThreadPool type.
//! This should be used in conjunction with TcpListener and TcpStream types
//! to effectively build and serve a working web server
#![allow(dead_code)]
use std::error;
use std::fmt;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// A trait to allow us to pass a Box type to a thread and have that thread grab the
/// corresponding value out of the Box.
trait FnBox {
    fn call_box(self: Box<Self>);
}

/// Implement the FnBox trait for a generic F type
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

/// Alias the stretched out type names
type Job = Box<FnBox + Send + 'static>;
type AMRMessage = Arc<Mutex<mpsc::Receiver<Message>>>;

/// Make a Message enum to send either a Job or a Terminate message
enum Message {
    NewJob(Job),
    Terminate,
}

/// A pool of native threads whose main job is to serve an HTTP request
pub struct ThreadPool {
    /// A container for tracking the working threads
    workers: Vec<Worker>,
    /// The sending end of a mpsc channel for giving work to the threads
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// A ThreadPool constructor. It expects an integer indicating the number of
    /// threads to create.
    pub fn new(size: usize) -> Result<ThreadPool, ThreadPoolError> {
        if size <= 0 || size > 15 {
            let msg = format!(
                "Bad value provided for a thread pool size. {} is not between 1 and 15",
                size
            );
            return Err(ThreadPoolError(msg));
        }
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Ok(ThreadPool { workers, sender })
    }
    /// Execute a Job in one of the threads inside the pool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

/// Implement the Drop trait to clean up the thread pool
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Asking threads to finish their work...");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("Shutting down worker {}...", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// A type acting as the holder of the Job that needs to be carried out
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: AMRMessage) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::Terminate => break,
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job.call_box();
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// An Error type to return when the ThreadPool creation fails
#[derive(Debug, Clone)]
pub struct ThreadPoolError(String);

/// Implement Display for our ThreadPoolError type
impl fmt::Display for ThreadPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl error::Error for ThreadPoolError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl From<&str> for ThreadPoolError {
    fn from(s: &str) -> Self {
        ThreadPoolError(String::from(s))
    }
}

#[cfg(test)]
mod tests {
    use super::ThreadPool;

    #[test]
    fn tp_wrong_size_zero() {
        let pool = ThreadPool::new(0);
        assert!(pool.is_err());
    }
    #[test]
    fn tp_wrong_size_gt_100() {
        let pool = ThreadPool::new(200);
        assert!(pool.is_err());
    }
    #[test]
    fn tp_good_size() {
        let pool = ThreadPool::new(2);
        assert!(pool.is_ok());
    }
}
