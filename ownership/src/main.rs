fn main() {
    let hola = String::from("hola");

    for i in 1..10 {
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

    let referencia_a_nada = dangle();
    println!("{referencia_a_nada}");


}

fn dangle() -> String {
    let s = String::from("hello");

    s
}
