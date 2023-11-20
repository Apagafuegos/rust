use std::io;
fn main() {

    for _n in 0..5{ 
        println!("Introduce los celsius que quieres transformar a Fahrenheit");
        let mut celsius = String::new();

        io::stdin().read_line(&mut celsius).expect("Que dices?"); //Lee el número a introducir

        let celsius: u32 = match celsius.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
                //Convierte el String en un i32
            };

        let fahrenheit = {
            celsius * 9/5 + 32
        };  
        println!("{celsius} grados Celsius son {fahrenheit} grados fahrenheit");
        println!();

    };    

}
