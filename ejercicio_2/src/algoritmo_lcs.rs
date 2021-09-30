/*
Longest Common Subsequence es un algoritmo conocido: dadas dos secuencias, 
¿cuál es la subsecuencia más larga que aparece en ambas?

Si las secuencias de caracteres son a b c d y a d b c, 
la subsecuencia común más larga es a b c, porque estos caracteres aparecen 
en ambas secuencias en ese orden (
notar que la subsecuencia no necesita ser consecutiva, sino que debe estar en orden).
*/
// https://www.youtube.com/watch?v=NnD96abizww -> video con la posta
pub mod longestcommonsubsequence{

    pub fn construir_diff(matriz_secuencias: Vec<Vec<i32>>, lineas_archivo1: &Vec<char>,
        lineas_archivo2: &Vec<char>, longitud_archivo1:usize, longitud_archivo2:usize){

        if longitud_archivo1 > 0 && longitud_archivo2 > 0 && 
        lineas_archivo1[longitud_archivo1 - 1] == lineas_archivo2[longitud_archivo2 - 1]{

            construir_diff(matriz_secuencias, lineas_archivo1, lineas_archivo2
                , longitud_archivo1-1, longitud_archivo2-1); //ok
        
            println!("> {}",lineas_archivo2[longitud_archivo1-1]);//ok

        }else if longitud_archivo2 > 0 && (longitud_archivo1 == 0 || matriz_secuencias[longitud_archivo1][longitud_archivo2-1] 
            >= matriz_secuencias[longitud_archivo1-1][longitud_archivo2]){

            construir_diff(matriz_secuencias, lineas_archivo1, lineas_archivo2
                , longitud_archivo1, longitud_archivo2-1); //ok
            println!("> {}",lineas_archivo2[longitud_archivo2-1]);// ok

        }else if longitud_archivo1 > 0 && (longitud_archivo2 == 0 || matriz_secuencias[longitud_archivo1][longitud_archivo2-1] 
            < matriz_secuencias[longitud_archivo1-1][longitud_archivo2]){

            construir_diff(matriz_secuencias, lineas_archivo1, lineas_archivo2
                , longitud_archivo1-1, longitud_archivo2); // ok
            println!("< {}",lineas_archivo1[longitud_archivo1-1]); //oj

        }else{
            println!("");
        }
    }

    // C es la grilla computada por lcs()
// X e Y son las secuencias
// i y j especifican la ubicacion dentro de C que se quiere buscar cuando 
//    se lee el diff. Al llamar a estar funcion inicialmente, pasarle
//    i=len(X) y j=len(Y) // i es long1 y j es long2
/*
function print_diff(C, X, Y, i, j)
if i > 0 and j > 0 and X[i-1] = Y[j-1]
    print_diff(C, X, Y, i-1, j-1)
    print "  " + X[i-1]
else if j > 0 and (i = 0 or C[i,j-1] >= C[i-1,j])
    print_diff(C, X, Y, i, j-1)
    print "> " + Y[j-1]
else if i > 0 and (j = 0 or C[i,j-1] < C[i-1,j])
    print_diff(C, X, Y, i-1, j)
    print "< " + X[i-1]
else
    print ""
*/

    pub fn creacion_matriz_secuencias(vector_lineas_archivo1: &Vec<char>, vector_lineas_archivo2: &Vec<char>) 
    -> Vec<Vec<i32>> {
        let largo_archivo1 = vector_lineas_archivo1.len();
        let largo_archivo2 = vector_lineas_archivo2.len();
        
        let mut matriz_secuencias: Vec<Vec<i32>> = Vec::<Vec<i32>>::new(); 

        for largo in 0..largo_archivo1+1{   //+1 porque no incluye el ultimo
            matriz_secuencias.push(Vec::<i32>::new());
        }

        
        for largo1 in 0..largo_archivo1+1{
            for largo2 in 0..largo_archivo2+1{
                matriz_secuencias[largo1].push(0);
            }
        }

        for i in 0..largo_archivo1{
            for j in 0..largo_archivo2{
                if vector_lineas_archivo1[i] == vector_lineas_archivo2[j]{
                    matriz_secuencias[i+1][j+1] = matriz_secuencias[i][j] + 1;
                }else{
                    if matriz_secuencias[i+1][j] > matriz_secuencias[i][j+1]{
                        matriz_secuencias[i+1][j+1] = matriz_secuencias[i+1][j];
                    }else{
                        matriz_secuencias[i+1][j+1] = matriz_secuencias[i][j+1];
                    }
                }
            }
        }

        matriz_secuencias
    }

    /*
    let X and Y be sequences
    let m be the length of X, and let n be the length of Y

    C = matriz_secuencias(m+1, n+1)
    // recordar que .., es inclusivo para el límite inferior, pero excluye al superior
    for i := 0..m+1
        C[i,0] = 0
    for j := 0..n+1
        C[0,j] = 0
    for i := 0..m
        for j := 0..n
            if X[i] = Y[j]
                C[i+1,j+1] := C[i,j] + 1
            else
                C[i+1,j+1] := max(C[i+1,j], C[i,j+1])

    return C
    */


}