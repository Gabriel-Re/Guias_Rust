/*
Enviar los task a un threadpool con un numero de hilos precreados
ejecutando las task enviandas una por una.

Threadpool nos permite controlar cuantos threads estan activos
al mismo tiempo

Hay una cola bloqueante para las task. 
Son las task a ser ejecutadas

Cuando un thread está libre va a buscar una task a la cola.

Si no hay task en la cola los threads van a esperar a que haya task.
Recordar que un solo task opera por task
*/

pub mod Threadpool{

    use std::sync::mpsc;
    use crate::threadmod;

    pub struct ThreadPool{
        threads: Vec<threadmod::threadmod::ThreadMod>,
        cantidad_hilos: u32,
        esta_funcionando: bool,
        sender: mpsc::Sender<()>,
        //vector_threads: Vec<thread>
        //Queue de tareas sin ejecutar
        //Cola de tareas ejecutandose?
        //Tareas completadas?
        //Estado del thread, libre o ocupado
        //vectores de productores y consumidores?
    }

    impl ThreadPool{

        pub fn new(cantidad_hilos:u32) -> Self{
            //if cant == 0 error?
            let mut vector_hilos = vec!();
            let (sender,receiver) = mpsc::channel();

            for i in 0..&cantidad_hilos+1{
                vector_hilos.push(threadmod::threadmod::ThreadMod::new(i));
            }

            ThreadPool{
                threads: vector_hilos,
                cantidad_hilos: cantidad_hilos,
                esta_funcionando: false,
                sender: sender
            }
        }
        
    }

}