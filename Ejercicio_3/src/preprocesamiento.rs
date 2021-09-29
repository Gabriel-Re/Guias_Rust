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


    //El que va a operar con los archivos va a ser el for de abajo
    /*
    pub fn operar_archivos(vector_rutas: Vec<&str>){
        let mut numero = 0;
        for ruta_archivo in vector_rutas{
            let mut archivo_leido = lectura_archivo(ruta_archivo); //lee perfecto
            numero += 1;
            println!("estamos en el archivo {}",numero);
            /*
            println!("{}",archivo_leido);
            println!("------------------------------------------------------------------------");
            println!("estamos en el archivo {}",numero);
            println!("------------------------------------------------------------------------");
            */
        }


    }
    */

    pub fn operar_archivos(vector_rutas: Vec<&str>) -> HashMap<String,Vec<i32>>{
        
        use crate::manejo_lineas;
                                    
        let mut hash_strings: HashMap::<String,Vec<i32>> = HashMap::<String,Vec<i32>>::new();
        let mut indice_archivo = 1;

        //deberia ser un doble for, en el cual este seria el 2do for y el primero es iterar los distintos archivos
        for ruta_archivo in vector_rutas{
            let archivo_leido = lectura_archivo(ruta_archivo); //lee perfecto
            for linea in archivo_leido.lines(){
                let strings_lineas: Vec<String> = manejo_lineas::manejo_linea::operar_lineas(linea);
                println!("La linea leida es {:?}",&strings_lineas);
                manejo_lineas::manejo_linea::hashear_palabras(strings_lineas,&mut hash_strings,indice_archivo);
            }
            indice_archivo += 1;
        }
        

        hash_strings
    }
}