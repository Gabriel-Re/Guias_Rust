pub mod manejo_linea{
    use std::collections::HashMap;
    pub fn operar_lineas(linea_leida: &str) -> Vec<String>{
        let mut linea_auxiliar = linea_leida.replace(",","");
        linea_auxiliar = linea_auxiliar.replace(".","");
        linea_auxiliar.split_whitespace().map(|aux| aux.to_string()).collect()
    }
   
    pub fn hashear_palabras(vector_strings_leidos: Vec<String>, hash_strings: &mut HashMap<String,i32>){
        for palabra in vector_strings_leidos{
            let contador_palabra = hash_strings.entry(palabra).or_insert(0);
            *contador_palabra += 1;
        }
    }

}