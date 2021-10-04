pub mod worker{

    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;
    use std::sync::mpsc;
    use crate::message;


    type Job = Box<dyn FnOnce() + Send + 'static>;

    pub struct Worker{
        pub thread: Option<thread::JoinHandle<()>>,
        pub id_thread: usize,
    }

    impl Worker{
        pub fn new(id_thread: usize, receiver: Arc<Mutex<mpsc::Receiver<message::message::Message>>>) -> Self{
            let thread = thread::spawn(move || loop {
                let message = receiver.lock().expect("El lock está envenenado").recv().expect("Error al obtener el trabajo");
                //Here, we first call lock on the receiver to acquire the mutex,
                // and then we call unwrap to panic on any errors.
    
                //The call to recv blocks, so if there is no job yet, the current 
                //thread will wait until a job becomes available. 
                //The Mutex<T> ensures that only one Worker thread at a time is trying to request a job.

                match message {
                    message::message::Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id_thread);
    
                        job();
                    }
                    message::message::Message::Terminate => {
                        println!("Worker {} was told to terminate.", id_thread);
    
                        break;
                    }
                }
            });
            Worker{
                thread: Some(thread),
                id_thread,
            }
        }
    }


}