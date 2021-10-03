pub mod taskmod{
   
    pub enum Estado_Trabajo{
        Libre(u32), //id: u32
        Ocupado(u32),//id: u32
        Trabajo_Finalizado(u32,u32)//id: u32, id_thread: u32
    } 

    pub struct TaskMod{
        //tarea
        id_task:u32,
        estado: Estado_Trabajo,
    }
}

