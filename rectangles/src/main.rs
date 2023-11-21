#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

#[derive(Debug)]
struct Triangle {
    base: u32,
    height: u32,
}
impl Triangle {
    //Implementa una funcionalidad a un tipo y devuelve un valor, o nada
    //En este caso, crea una funcion area que llama a los valores de sí mismo para calcular el área del triangulo
    fn area(&self) -> u32 {
        (self.base * self.height) / 2
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rect1 is {:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let tri1 = Triangle {
        base: 10,
        height: 33,
    };

    println!("Tri1 is {:?}", tri1);
    println!("El área del triangulo es {} cm2", tri1.area());
}
