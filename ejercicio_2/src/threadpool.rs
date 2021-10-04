/*
execute will send a job from the ThreadPool to the Worker 
instances, which will send the job to its thread. 

Here is the plan:

1-The ThreadPool will create a channel and hold on to the 
sending side of the channel.
    
2- Each Worker will hold on to the receiving side of the channel.
    
3-We’ll create a new Job struct that will hold the closures we want
 to send down the channel.
    
4- The execute method will send the job it wants to execute down 
 the sending side of the channel.

5- In its thread, the Worker will loop over its receiving 
side of the channel and execute the closures of any jobs it receives.
*/

pub mod Threadpool{

    use std::sync::mpsc;
    use std::sync::Arc;
    use std::sync::Mutex;
    use crate::worker;
    use crate::message;
    
    type Job = Box<dyn FnOnce() + Send + 'static>;
    /*
    we need Send to transfer the closure from one thread to another and 'static because 
    we don’t know how long the thread will take to execute
    */

    pub struct ThreadPool{
        workers: Vec<worker::worker::Worker>,
        sender: mpsc::Sender<message::message::Message>,
    }

    impl ThreadPool{

        pub fn new(cantidad_hilos:usize) -> Self{

            assert!(cantidad_hilos > 0); // Panic si es 0

            let mut vector_workers =  Vec::with_capacity(cantidad_hilos);
            let (sender,receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            /*
            The Arc type will let multiple workers own the receiver, and Mutex 
            will ensure that only one worker gets a job from the receiver at a time
            */

            for i in 0..cantidad_hilos{
                vector_workers.push(worker::worker::Worker::new(i,Arc::clone(&receiver)));
            }
            //we clone the Arc to bump the reference count so the workers can share ownership 
            //of the receiving end.

            ThreadPool{
                workers: vector_workers,
                sender: sender,
            }
        }

        pub fn execute<F>(&self, f: F)
            where
                F:FnOnce() + Send + 'static,
            {
                let job = Box::new(f);

                self.sender.send(message::message::Message::NewJob(job)).unwrap();
            }
        
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            println!("Sending terminate message to all workers.");
    
            for _ in &self.workers {
                self.sender.send(message::message::Message::Terminate).unwrap();
            }
    
            println!("Shutting down all workers.");
    
            for worker in &mut self.workers {
                println!("Shutting down worker {}", worker.id_thread);
    
                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }

}