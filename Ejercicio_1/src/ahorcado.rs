pub mod ahorcado{
    pub struct Ahorcado{
        pub letras_adivinadas: Vec<char>,
        pub palabra_a_adivinar: String,
        pub letras_ingresadas: Vec<char>,
        pub vidas_jugador: i32,
        pub juego_terminado: bool
    }
        
    impl Ahorcado{
        pub fn crear_ahorcado(palabra_ingresada: String) -> Self{
    
            let letras_adivinadas: Vec<char> = vec!['_'; palabra_ingresada.len()];
            let palabra_a_adivinar: String = palabra_ingresada.to_ascii_lowercase();
            Ahorcado{
                letras_adivinadas,
                palabra_a_adivinar,
                letras_ingresadas: Vec::new(),
                vidas_jugador: 5,
                juego_terminado: false
            }
        }
    
        pub fn perder_una_vida(&mut self){
            self.vidas_jugador = self.vidas_jugador - 1;
        }

        pub fn operar_letra_ingresada(&mut self, letra_ingresada: char){

            if self.es_letra_ya_ingresada(&letra_ingresada) == false && self.juego_terminado == false{
                self.buscar_letra_palabra_secreta(letra_ingresada);
                self.letras_ingresadas.push(letra_ingresada.to_ascii_lowercase());  
            }else{
                println!("La letra ya fue adivinada, por favor ingresa otra");
            }
            self.chequear_estado_juego();
        }

        pub fn buscar_letra_palabra_secreta(&mut self, letra_ingresada: char){

            let mut indices_encontrados = vec![];
            let mut indice_actual = 0;
            let mut encontro_letra = false;

            for letra in self.palabra_a_adivinar.chars(){
                if letra.eq_ignore_ascii_case(&letra_ingresada){
                    encontro_letra = true;
                    indices_encontrados.push(indice_actual);
                    println!("aguanteboca");
                }
                println!("la letra iterada es {}",&letra);
                indice_actual+= 1;
            }

            if encontro_letra == true{
                println!("Hay letra que coincide");
                self.actualizar_palabra_buscada(indices_encontrados,letra_ingresada);
            }else{
                println!("Perdiste una vida maquinola");
                self.perder_una_vida();
            }
        }
    
        pub fn actualizar_palabra_buscada(&mut self,indices: Vec<usize>, letra_adivinada: char){
            let tamanio_array = indices.len();
            
            for i in 0..tamanio_array{
                self.letras_adivinadas.remove(indices[i]);
                self.letras_adivinadas.insert(indices[i],letra_adivinada);
            }
        }

        pub fn es_letra_ya_ingresada(&self, letra_ingresada: &char) -> bool{
            for letra in &self.letras_ingresadas{
                if letra.eq_ignore_ascii_case(letra_ingresada){
                    return true;
                }
            }
            false
        }

        pub fn chequear_estado_juego(&mut self){
            let string_adivinado = (self.letras_adivinadas).clone().iter().collect::<String>();
            if string_adivinado.eq_ignore_ascii_case(&self.palabra_a_adivinar){
                self.juego_terminado = true;
            }
        }

    }
    
}



