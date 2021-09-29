pub mod manejo_linea{
    use std::collections::HashMap;
    pub fn operar_lineas(linea_leida: &str) -> Vec<String>{
        let mut linea_auxiliar = linea_leida.replace(",","");
        linea_auxiliar = linea_auxiliar.replace(".","");
        linea_auxiliar.split_whitespace().map(|aux| aux.to_string()).collect()
    }
   
    pub fn hashear_palabras(vector_strings_leidos: Vec<String>, hash_strings: &mut HashMap<String,Vec<i32>>, 
                            id_archivo: i32){
        
        for palabra in vector_strings_leidos{
            let mut vector_aux = vec!();
            vector_aux.push(id_archivo);

            if !hash_strings.contains_key(&palabra){
                hash_strings.entry(palabra).or_insert(vector_aux);
            }else{
                let vector_clave = hash_strings.entry(palabra).or_insert(vector_aux);
                for id_vector in vector_clave.iter(){
                    if id_vector != &id_archivo{
                        vector_clave.push(id_archivo);
                    }
                }
            }
    
            
        }
    }

}