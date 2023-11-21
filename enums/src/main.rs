#[derive(Debug)]
enum CategoriasFIA {
    Monoplazas,
    Turismos,
    HyperCars,
    Rally,
}
struct Categoria {
    kind: CategoriasFIA,
    champion: String,
}
fn main() {
    let f1 = Categoria {
        kind: CategoriasFIA::Monoplazas,
        champion: String::from("Max Verstappen"),
    };
    println!("{:?}, {:?}", f1.kind, f1.champion);

    let wec = Categoria {
        kind: CategoriasFIA::HyperCars,
        champion: String::from("Toyota Gazoo Racing"),
    };
    println!("{:?},{:?}", wec.kind, wec.champion);

    let gt3: Categoria = Categoria {
        kind: CategoriasFIA::Turismos,
        champion: String::from("Carlos Santos"),
    };
    println!("{:?},{:?}", gt3.kind, gt3.champion);

    let dakar = Categoria {
        kind: CategoriasFIA::Rally,
        champion: String::from("Carlos Sainz"),
    };
    println!("{:?},{:?}", dakar.kind, dakar.champion);
}
