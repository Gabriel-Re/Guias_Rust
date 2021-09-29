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

    pub fn lcs(){

    }

    pub fn creacion_matriz_secuencias(vector_lineas_archivo1: &Vec<String>, vector_lineas_archivo2: &Vec<String>) 
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
                    if(matriz_secuencias[i+1][j] > matriz_secuencias[i][j+1]){
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

    C = grid(m+1, n+1)
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