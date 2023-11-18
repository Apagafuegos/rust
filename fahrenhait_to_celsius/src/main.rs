use std::io;
fn main() {

    loop{ 
        let mut cont = 0;
        println!("Introduce los celsius que quieres transformar a Fahrenheit");
        let mut celsius = String::new();

        io::stdin().read_line(&mut celsius).expect("Que dices?");

        let celsius: u32 = match celsius.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

        let fahrenheit = celsius * 9/5 + 32; 
        println!("{celsius} grados Celsius son {fahrenheit} grados fahrenheit");
        cont += 1;

        if cont > 5{
            break;
        }
    };    

}
