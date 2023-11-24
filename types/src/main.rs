fn main() {
    let lista_numeros = vec![33, 14, 54, 55, 44];
    let resultado = mayor(&lista_numeros);

    println!("El numero más grande es {resultado}");

    let lista_numeros = vec![0, 14, 3, 5, 6, 7];
    let resultado = mayor(&lista_numeros);

    println!("El numero más grande es {resultado}");
    //Se pueden hacer structs y enums que almacenen cualquier tipo de dato genérico
    let _punto_integer = Point{x:5, y:5};
    let _punto_float = Point{x:5.3, y:3.3};
    //Da error si el struct no implementa 2 tipos distintos -> let _punto_mix = Point{x:5.3, y:3};
    let p = Point{x: 5.3 , y: 33.3};
    let p2 = Point{x:5, y:3};
    println!("La coordenada x del punto es {}",p.x());
    println!("La coordenada y del punto es {}", p.y());
    println!("La distancia al origen es de {} ud.", p.distancia_origen());

    println!("La coordenada x del punto es {}",p2.x());
    println!("La coordenada y del punto es {}", p2.y());
    println!("La distancia al origen es de {} ud.", p2.distancia_origen());
}

fn mayor(lista: &[i32]) -> &i32 {
    let mut mayor = &lista[0];
    for number in lista {
        if number > mayor {
            mayor = number;
        }
    }
    return mayor;
}

struct Point<T>{
    x: T,
    y: T,
} //Defino un tipo llamado punto con dos campos de dos tipos distintos


impl Point<f32>{ //Implemento el tipo custom en estas dos funciones
    fn x(&self) -> &f32 {
        &self.x //"Saco" la componente X del tipo punto 
    }
    fn y(&self) -> &f32 {
        &self.y //Igual, pero con la Y
    }

    fn distancia_origen(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    
}

impl Point<i32> {
    fn x(&self) -> &i32 {
        &self.x //"Saco" la componente X del tipo punto 
    }
    fn y(&self) -> &i32 {
        &self.y //Igual, pero con la Y
    }

    fn distancia_origen(&self) -> f32 {
        (self.x.pow(2)as f32 + self.y.pow(2)as f32).sqrt()
    }
}
