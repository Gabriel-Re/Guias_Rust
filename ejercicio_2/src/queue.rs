pub mod queue{
    pub struct Queue<T>{
        queue: Vec<T>,
        tamanio: u32,
    }

    impl<T> Queue<T>{
        pub fn new() -> Self{
            Queue{
                queue: Vec::new(),
                tamanio: 0
            }
        }

        pub fn agregar_tarea(&mut self, task: T){
            self.queue.push(task);
            self.tamanio +=1;
        }

        pub fn procesar_tarea(&mut self)-> Result<T,String>{
            if self.tamanio == 0{
                return Err(String::from("No hay tareas para procesar"));
            }
            self.tamanio -= 1;
            let task_a_procesar = self.queue.remove(0);
            Ok(task_a_procesar)
        }


    }
    
}