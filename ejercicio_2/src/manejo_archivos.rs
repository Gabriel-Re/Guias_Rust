pub mod operar_archivos{

    pub fn read_files_lines(ruta: &str) -> Vec<String>{
        use std::fs;
        let mut archivo_leido = fs::read_to_string(ruta).expect("Unable to read file");
        archivo_leido = archivo_leido.to_ascii_lowercase();

        let mut vector_strings: Vec<String> = vec!();

        for linea in archivo_leido.lines(){
            let strings_linea:String = linea.to_string();
            vector_strings.push(strings_linea);
        }
        vector_strings

    }

}

