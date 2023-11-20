fn main() {
    let hola = String::from("hola");

    for i in 1..100 {
        let a = i + 2;
        println!("{}", a);
    }
    //print!("{a}");
    //let s2 = hola;
    //println!("{}", hola);no funciona ya que si copias directamente Rust elimina la variable hola
    //para que funcionase habria que clonar
    let _s2 = hola.as_str();
    println!("{hola}");
    //print!("{}", a); fuera del scope no existe el valor
    let y = 5;
    let _x = y;
    print!("{y}");
}
