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

    let wec = Categoria {
        kind: CategoriasFIA::HyperCars,
        champion: String::from("Toyota Gazoo Racing"),
    };
}
