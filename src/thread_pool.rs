use std::thread;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::Arc;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
impl ThreadPool {
    pub fn new(num_threads: usize)->ThreadPool{
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(num_threads);
        for id in 0..num_threads {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool{
            workers,
            sender,
        }
    }
    pub fn execute<F>(&self,f:F)
        where 
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self){
        println!("Drop the threads");
        for _ in &mut self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers{
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->Worker {
        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::Terminate => break,
                    Message::NewJob(job) => job(),
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    Terminate,
    NewJob(Job),
}

type Job = Box<dyn FnOnce() + Send + 'static>;
