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
   
    let vector1: Vec<String> = manejo_archivos::operar_archivos::read_files_lines(RUTA1);
    let vector2: Vec<String> = manejo_archivos::operar_archivos::read_files_lines(RUTA2);

    let matriz_secuencias: Vec<Vec<i32>> = algoritmo_lcs::longestcommonsubsequence::creacion_matriz_secuencias(&vector1, &vector2);

}
