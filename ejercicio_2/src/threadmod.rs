pub mod threadmod{
    use std::thread;
    pub struct ThreadMod{
        thread: thread::JoinHandle<()>,
        id_thread: u32,
    }

    impl ThreadMod{
        pub fn new(id_thread: u32) -> Self{

            ThreadMod{
                thread: thread::spawn(|| {}),
                id_thread,
            }
        }
    }


}