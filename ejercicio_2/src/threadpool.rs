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

    use std::task;

    pub struct ThreadPool{
        cantidad_hilos: u32,
        //vector_threads: Vec<thread>
        //Queue de tareas sin ejecutar
        //Cola de tareas ejecutandose?
        //Tareas completadas?
        //Estado del thread, libre o ocupado
    }

    impl ThreadPool{

        pub fn new(cantidad_hilos:u32) -> Self{
            use std::thread;
            let mut vector_hilos = vec!();

            for i in 0..&cantidad_hilos+1{
                vector_hilos.push(thread::spawn(move ||{
                    println!("this is thread number {}", i);
                }));
            }

            ThreadPool{
                cantidad_hilos: cantidad_hilos
                //vector_hilos: vector_hilos
            }
        }
        
        /*
        pub fn chequear_cantidad_hilos(self){
            //if cantidad hijos < minima_cant_hijos{
                // creo hijos segun cuantos falten
            //}
        }
        */



    }

}