/*
Encontrar la diferencia entre dos archivos es un problema que es bastante conocido y estudiado.
La mayoría de las implementaciones usan el algoritmo de Myers, en este ejercicio, 
haremos que calcule la subsecuencia común más larga entre los dos archivos con el algoritmo 
LCS y use esa información para calcular su diferencia.
*/

const RUTA1: &str = "/home/gabore/Escritorio/Facu/Taller/Guias_Rust/Guia_2/ejercicio_2/src/textos_de_ejemplo/textoejemplo_1.txt"; 
const RUTA2: &str = "/home/gabore/Escritorio/Facu/Taller/Guias_Rust/Guia_2/ejercicio_2/src/textos_de_ejemplo/textoejemplo_2.txt"; 
mod manejo_archivos;
mod algoritmo_lcs;


fn main() {
   
    let lineas_archivo1: Vec<String> = manejo_archivos::operar_archivos::read_files_lines(RUTA1);
    let lineas_archivo2: Vec<String> = manejo_archivos::operar_archivos::read_files_lines(RUTA2);
    
    let largo_archivo1 = lineas_archivo1.len();
    let largo_archivo2 = lineas_archivo2.len();

    let mut largo_iterable = 0;

    if largo_archivo1 <= largo_archivo2{
        largo_iterable = largo_archivo1;
    }else{
        largo_iterable = largo_archivo1;
    }

    for i in 0..largo_iterable{
        let lista_archivo1:Vec<char> = lineas_archivo1[i].chars().collect();
        let long1 = lista_archivo1.len();
        let lista_archivo2:Vec<char> = lineas_archivo2[i].chars().collect();
        let long2 = lista_archivo2.len();        
        
        let mut matriz_secuencias: Vec<Vec<i32>> = algoritmo_lcs::longestcommonsubsequence::creacion_matriz_secuencias(&lista_archivo1, &lista_archivo2);
        

        algoritmo_lcs::longestcommonsubsequence::construir_diff(matriz_secuencias, &lista_archivo1, 
            &lista_archivo2, largo_archivo1, largo_archivo2);

    }

    



}
