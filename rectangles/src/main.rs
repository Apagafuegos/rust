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

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_larger_hold_smaller(){
        let larger = Rectangle{
            width: 5,
            height: 6,
        };
        let smaller = Rectangle{
            width: 3,
            height: 2,
        };
        
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn cant_larger_hold_smaller(){
        let larger = Rectangle{
            width: 5,
            height: 6,
        };
        let smaller = Rectangle{
            width: 10,
            height: 23,
        };
        
        assert!(smaller.can_hold(&larger));
    }


}