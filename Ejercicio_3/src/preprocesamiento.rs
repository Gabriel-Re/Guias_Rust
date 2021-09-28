/*
Otra forma de ordenar esto es crear un mod que sea preprocesado y dentro
de ese mod definir los mod de manejo de archivos y procesado de lineas
*/
pub mod manejo_archivos{
    use std::collections::HashMap;

    pub fn lectura_archivo(ruta: &str)-> String{
        use std::fs;
        let archivo_leido = fs::read_to_string(ruta).expect("Unable to read file");
        archivo_leido.to_ascii_lowercase()
    }

    pub fn operar_archivos(vector_rutas: Vec<&str>){

        for ruta_archivo in vector_rutas{
            lectura_archivo(ruta_archivo);
        }

    }

    /*
    pub fn operar_archivo(archivo_leido: String) -> HashMap<String,i32>{//puede retornar un hashmap con las weas ordenadas
        use crate::manejo_lineas;   //sin esto no podria usar el manejo de linea, por otro lado 
                                    // El mod de manejo de lineas está importado en el main
                                    
        let mut hash_strings: HashMap::<String,i32> = HashMap::<String,i32>::new();
                                 
        for linea in archivo_leido.lines(){
            let strings_lineas: Vec<String> = manejo_lineas::manejo_linea::operar_lineas(linea);
            println!("La linea leida es {:?}",&strings_lineas);
            manejo_lineas::manejo_linea::hashear_palabras(strings_lineas,&mut hash_strings);

        }

        hash_strings
    }
    */
}