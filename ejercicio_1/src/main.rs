/*
Ejercicio 1.

Analizar las siguientes porciones de código y responder si el mismo compila o no. 
Explicar por qué sí o por qué no.
Si no se compila, ¿qué podrías cambiar para que compile? 
*/

/*
// Funcion drip_drop

fn drip_drop() -> &String{
    let s = String::from("hello world!");
    return &s   // El problema es que estamos retorando alque despues de este scope desaparece
}
*/




/*
// Main 1

fn main() {
    let mut s = String::from("hola");
    let ref1 = &s;
    let ref2 = &ref1; 
    let ref3 = &ref2; 
    s = String::from("chau"); // -> solucion: definir las referencias despues de esta linea

    println!("{}", ref2.to_uppercase());  
     // Esta parte causa problemas, estamos creando varias 
    // referencias a un string que luego es remplazado; por lo tanto
    // dejamos de tener una referencia válida a esa variable,
    // ya que fue reemplazada
}
*/


/*
// Main 2

fn main() {
    let s1 = String::from("hola");
    let mut v = Vec::new();
    v.push(s1); // -> Posible solución, pushear una copia del string
    let s2: String = v[0];  //Estamos tratando de tener un mismo ownership en dos referencias
    println!("{}", s2);
}
*/
