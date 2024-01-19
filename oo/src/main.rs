mod reloj;
use crate::reloj::Reloj;

fn main() {
    let mut prueba: Reloj = Reloj::new(17, 17, 17, false);
    println!("{}", prueba.to_string());
    prueba.poner_en_hora(18, 30, 50);
    println!("{}", prueba.to_string());
    let _reloj1 = prueba.clone();
}
