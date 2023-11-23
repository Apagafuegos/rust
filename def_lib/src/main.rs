pub mod lib;
use std::collections::HashMap;
fn main() {
    //let v: Vec<i32> = Vec::new();
    //let v = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(33);
    v.push(14);
    v.push(55);

    let third_elem = &v[2];
    println!("El tercer elemento es {third_elem}");

    let fourth_elem = v.get(3);
    match fourth_elem {
        Some(fourth_elem) => println!("El cuarto elemento es {fourth_elem}"),
        None => println!("No hay cuarto elemento en el vector"),
    }
    let no_existe = v.get(3);
    println!("{:?}", no_existe);

    let mut v2 = vec![1, 2, 3, 4, 5];
    let first_elem = &v2[0];
    //v2.push(6); no permite pedir prestado un elemento y luego añadir otro debido al memory allocation
    println!("El primer elemento es {first_elem}");

    for i in &v2 {
        print!("{i} ");
    }
    println!();
    for e in &mut v2 {
        *e += 50; // * es dereference
        print!("{e} ");
    }

    let row = vec![
        lib::TipoDato::Int(3),
        lib::TipoDato::Float(33.33),
        lib::TipoDato::Text(String::from("Fernando Alonso")),
    ];
    println!();
    for j in row {
        print!("{:?}", j);
    }
    println!();

    let mut scores = HashMap::new(); //Diccionario con clave : valor en python

    scores.insert(String::from("Alonso"), 14);
    scores.insert(String::from("Sainz"), 55);
    scores.insert(String::from("Hamilton"), 44);

    let piloto = String::from("Alonso");
    let numero_piloto = scores.get(&piloto).copied();
    println!("{}{:?}", piloto, numero_piloto);
}
