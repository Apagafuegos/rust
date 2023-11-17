use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        println!("Adivina el numero jeje");
    println!("Pero primero introducelo");
    let mut numero = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);


    io::stdin()
        .read_line(&mut numero)
        .expect("No he podido leer la linea bebe");

    print!("Tu dices que es {numero}...");

    println!("The secret number is {secret_number}" );

    let numero: u32 = match numero.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match numero.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => { 
            println!("You win!"); 
            break;
    }

    }
    }
    


}
