use std::io;
mod ahorcado;

mod manejo_archivo{

    pub fn lectura_archivo(lista_a_cargar: &mut Vec<String>){
        use std::fs;
        let data = fs::read_to_string("/home/gabore/Escritorio/Facu/Taller/Guia_1/Ejercicio_1/src/Palabras.txt").expect("Unable to read file");

        let mut i = 0;

        for line in data.lines(){
            lista_a_cargar.insert(i,line.to_string());
            i += 1;
        }

    }

}

fn main() {

    let mut lista_palabras = Vec::new();

    manejo_archivo::lectura_archivo(&mut lista_palabras);

    let primer_palabra = lista_palabras[0].clone();

    let mut juego_ahorcado = ahorcado::ahorcado::Ahorcado::crear_ahorcado(primer_palabra);

    println!("Letras adivinadas: {:?}",juego_ahorcado.letras_adivinadas);
    //println!("Palabra a adivinar: {:?}",juego_ahorcado.palabra_a_adivinar);

    let mut letra_ingresada = String::new();

    while juego_ahorcado.vidas_jugador >= 1 && juego_ahorcado.juego_terminado == false{
        println!("Letras adivinadas: {:?}",juego_ahorcado.letras_adivinadas);
        println!("Ingresa la letra troesma");
        io::stdin().read_line(&mut letra_ingresada).expect("Error leyendo la linea.");
        letra_ingresada.pop();
        println!("La letra ingresada es: {}", letra_ingresada);
        let letra:char = letra_ingresada.chars().next().unwrap();
        ahorcado::ahorcado::Ahorcado::operar_letra_ingresada(&mut juego_ahorcado, letra.to_ascii_lowercase()); //ingresa bien
        println!("Las letras ingresadas son: {:?}",juego_ahorcado.letras_ingresadas);
        letra_ingresada.pop();
    }

    if juego_ahorcado.juego_terminado == true{
        println!("Felicitaciones troesma, sos un crack, te ganaste nada pero terminaste el ejercicio");
    }else{
        println!("Mal fla perrito malvado, la prox será");
    }

}
