const RUTA: &str = "/home/gabore/Escritorio/Facu/Taller/Guia_1/Ejercicio_2/contar_palabras/ejemplo.txt"; 
mod manejo_archivos;
mod manejo_lineas;
use std::collections::HashMap;

fn main(){

    let archivo_leido = manejo_archivos::manejo_archivo::lectura_archivo(RUTA);

    let hash_strings: HashMap::<String,i32> = manejo_archivos::manejo_archivo::operar_archivo(archivo_leido);
    
    let mut vector_tuplas_palabra_valor: Vec<(&String,&i32)> = hash_strings.iter().collect();
    println!("{:?}",&vector_tuplas_palabra_valor);
    vector_tuplas_palabra_valor.sort_by(|a, b| b.1.cmp(a.1));
    for (clave, valor) in vector_tuplas_palabra_valor.iter(){
        println!("{} -> {}" ,clave, valor);
    }
}
