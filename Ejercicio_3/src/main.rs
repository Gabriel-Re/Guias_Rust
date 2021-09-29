const RUTA1: &str = "/home/gabore/Escritorio/Facu/Taller/Guias_Rust/Ejercicio_3/corpus/1_el_golazo_de_Messi.txt"; 
const RUTA2: &str = "/home/gabore/Escritorio/Facu/Taller/Guias_Rust/Ejercicio_3/corpus/2_dolar_precio_alto.txt"; 
const RUTA3: &str = "/home/gabore/Escritorio/Facu/Taller/Guias_Rust/Ejercicio_3/corpus/3_Messi_y_Guardiola.txt"; 

use std::collections::HashMap;
mod preprocesamiento;
mod manejo_lineas;

fn main() {

    let mut vector_archivos = vec!();

    vector_archivos.push(RUTA1);
    vector_archivos.push(RUTA2);
    vector_archivos.push(RUTA3);

    let hash_strings: HashMap::<String,Vec<i32>> =preprocesamiento::manejo_archivos::operar_archivos(vector_archivos);

    for (clave,vector) in hash_strings{
      println!("Clave: {}, Vector:{:?}",clave,vector);
    }

}
