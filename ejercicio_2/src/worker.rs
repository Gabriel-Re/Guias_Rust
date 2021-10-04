pub mod worker{

    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;
    use std::sync::mpsc;


    type Job = Box<dyn FnOnce() + Send + 'static>;

    pub struct Worker{
        thread: thread::JoinHandle<()>,
        id_thread: usize,
    }

    impl Worker{
        pub fn new(id_thread: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self{

            Worker{
                thread: thread::spawn(|| {}),
                id_thread,
            }
        }
    }


}