#![allow(special_module_name)]
pub mod lib;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    let mut cont = 0;
    loop {
        println!("Adivina el numero jeje");
        print!("Pero primero introducelo ");
        _ = io::stdout().flush();
        let mut numero = String::new();

        _ = io::stdin().read_line(&mut numero);

        let numero: i32 = match numero.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = lib::Guess::new(numero); /*Creo una variable guess de tipo guess para que evalue si el numero
                                             introducido esta en los limites que quiero, si es asi, me devuelve un tipo guess{valor} al cual le extraigo ese valor
                                             y se lo introduzco a la variable numero usando la funcion value de mi libreria
                                             Luego, si el numero no esta en los limites que quiero, simplemente hago un panic!*/
        let numero = lib::Guess::value(&guess);

        print!("Tu dices que es {numero}...");

        cont += 1;
        match numero.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("It took you {} tries", &cont);
                break;
            }
        }
    }
}
