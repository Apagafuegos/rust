use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut cont = 0;
    let _array = vec![1, 2, 3];
    let _array2 = [3, 2, 5];
    loop {
        println!("Adivina el numero jeje");
        print!("Pero primero introducelo ");
        _ = io::stdout().flush();
        let mut numero = String::new();

        _ = io::stdin().read_line(&mut numero);

        print!("Tu dices que es {numero}...");

        let numero: u32 = match numero.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        cont += 1;
        match numero.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("It took you {} tries", cont);
                break;
            }
        }
    }

    for element in _array {
        println!("{}", element);
    }
}
