use std::{
    thread,
    sync::{mpsc,Arc,Mutex},
};
pub struct ThreadPool{
    
    workers:Vec<Worker>,
    sender:mpsc::Sender<Job>,
}
//Job存储要发送的闭包，execute发送闭包，Worker接收闭包并执行
type Job=Box<dyn FnOnce()+Send+'static>;
impl ThreadPool{
    /// Create a new ThreadPool.panic if the size is zero.
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);
        let (sender,recevier)=mpsc::channel();
        let recevier=Arc::new(Mutex::new(recevier));
        let mut workers=Vec::with_capacity(size);
        for id in  0..size{
            workers.push( Worker::new(id,Arc::clone(&recevier)));
        }
        

        ThreadPool {workers,sender}
    }
    

    

    pub fn execute<F>(&self, f:F)
    where
        F:FnOnce()+Send+'static,
    {
        let job=Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker{
    id:usize,
    thread:thread::JoinHandle<()>,
}
impl Worker{
    fn new(id:usize,recevier:Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread=thread::spawn(move || loop {
            let job=recevier.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
        });
    
        Worker{id, thread}
    }
}