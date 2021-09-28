/*
Ejercicio 3 - Buscador Full-text

La búsqueda de texto está en todos lados. 
Desde encontrar un mensaje en redes sociales, 
productos en portales de comercio electrónico, o cualquier otra cosa en la web a
 través de Google.

En este ejercicio, construiremos un motor de búsqueda sencillo que pueda buscar 
en millones de documentos y clasificarlos según su relevancia.

El primer paso consiste en la preparación de los datos. Necesitamos construir el 
conjunto de datos sobre el que realizaremos las búsquedas, denominado corpus. 
Este conjunto será un grupo de archivos de texto plano (txt) que puede generarse a 
partir de artículos de Internet. Cada archivo será un documento que estará 
identificado por un id.

Luego se debe realizar la indexación: Se debe implementar una estructura conocida
 como de índice invertido. Que será una estructura de datos de tipo HashMap que 
 contendrá como clave cada una de las palabras y como valor, el o los ids de 
 documentos en los que aparece la palabra. Para este paso, se debe realizar el 
 proceso de tokenización, es decir, obtener cada una de los tokens que conforman 
 al documento, considerando las separaciones de los mismos por espacios en blanco 
 o saltos de línea, y quitando los signos de puntuación. De estos tokens, 
 se debe ignorar las palabras más usadas del lenguaje español (conocidas como stop words), 
 por ejemplo, los artículos: la, el, las, los. Se debe considerar la frecuencia de cada token, 
 es decir, la cantidad de veces que el mismo aparece en el documento. Ese valor debe ser
  almacenado para el ordenamiento de los resultados.

El último paso es implementar la búsqueda. Para ello, se debe solicitar al usuario una
 frase a buscar y aplicar la tokenización de la misma y la eliminación de las stop words.
  Se debe buscar los documentos que contengan los términos de búsqueda ingresados.

Luego se debe determinar la relevancia de cada documento resultado de la búsqueda. P
ara esto, se debe determinar el puntaje del documento. Esto se puede computar a partir de
 sumar las frecuencias de cada uno de los términos encontrados.

Para mejorar el cálculo de puntaje del documento, calcularemos la frecuencia inversa 
de documentos para un término (denominado tf-idf) dividiendo la cantidad de documentos 
(N) en el índice por la cantidad de documentos que contienen el término, y tomaremos el 
logaritmo. 
*/

fn main() {

    let vector_archivos = vec!();


    println!("Hello, world!");
}
