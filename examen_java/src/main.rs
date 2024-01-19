use rand::distributions::*;
use std::io::Write;

fn main() {
    //ejercicio_1();
    //ej2();
    //ej3();
    ej4();
}

#[allow(dead_code)]
fn ejercicio_1() {
    //Generar un array de un tamaño entre 10 y 20 (aleatorio e impar), rellenarlo con los numeros del 50 al 1 y en las posiciones
    //impares poner la suma total de los valores
    let tamano = aux_ej1();
    let mut array_numeros = Vec::new();
    let mut contador = 50;

    for _i in 0..=tamano {
        array_numeros.push(contador);
        contador -= 1;
    }
    println!("Array inicial");
    for i in 0..array_numeros.len() {
        print!("{} ", array_numeros[i]);
    }

    let mut suma = 0;
    for i in 0..array_numeros.len() {
        suma += array_numeros[i];
    }
    println!();
    println!("La suma es {}", suma);

    for i in 0..array_numeros.len() {
        if i % 2 == 1 {
            array_numeros[i] = suma;
        }
    }
    println!("Array modificado");
    for i in 0..array_numeros.len() {
        print!("{} ", array_numeros[i]);
    }
}
#[allow(dead_code)]
fn ej2() {
    #[allow(unused_assignments)]
    let mut palabra: String = String::new();
    loop {
        println!("Introduce una palabra que empiece por F y que tenga mas de 7 letras:");
        _ = std::io::stdout().flush();
        _ = std::io::stdin().read_line(&mut palabra);
        if palabra.starts_with("F") && palabra.len() - 2 > 7 {
            break;
        }
    }
    let mut letras = palabra.split("").collect::<Vec<_>>();
    let random = aux_ej2(palabra.len() as i32);
    let random_string = random.to_string();

    letras.insert(random as usize, &random_string);
    println!("{}", letras[0]);
    for e in letras {
        print!("{e} ");
    }
}
/*
fn ej3() {
    println!("Introduce un numero largo");
    _ = std::io::stdout().flush();
    let mut numero = String::new();
    _ = std::io::stdin().read_line(&mut numero);

    let numeros = numero.split("").collect::<Vec<_>>();
    #[allow(unused_assignments)]
    let mut cuenta = 1;
    let mut aux: usize = 0;
    for i in aux..numeros.len() {
        cuenta = 1;
        for j in i..numeros.len() {
            if numeros[i] == numeros[j] {
                cuenta += 1;
            } else {
                aux = j;
                break;
            }
        }

        if cuenta > 1 {
            println!(
                "El numero {} se repite seguidamente {cuenta} veces.",
                numeros[i]
            );
        }
    }
}
*/

fn ej4() {
    let letras = vec!["A", "T", "F", "S", "L"];
    let mut palabra = String::new();
    let mut seguir = false;
    loop {
        println!("Introduce una palabra");
        _ = std::io::stdout().flush();
        _ = std::io::stdin().read_line(&mut palabra);
        palabra = String::from(palabra.trim());
        let letras_palabra = palabra.split("").collect::<Vec<_>>();

        for (e, i) in letras.iter().enumerate() {
            if &letras_palabra[1].to_ascii_uppercase() == i {
                println!(
                    "La palabra {palabra} empieza por la letra {i} en la posicion {e} del array."
                );
                seguir = true;
                break;
            }
        }

        if seguir {
            break;
        }
        println!("La palabra introducida no empieza por ninguna de las letras del array");
        palabra = String::new();
    }
}

//Auxiliares
#[allow(dead_code)]
fn aux_ej1() -> i32 {
    let mut rng = rand::thread_rng();
    let opciones = Uniform::from(10..21);
    loop {
        let tamano = opciones.sample(&mut rng);
        if tamano % 2 == 1 {
            return tamano;
        }
    }
}
#[allow(dead_code)]
fn aux_ej2(longitud: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let opciones = Uniform::from(0..=longitud);
    loop {
        let posicion = opciones.sample(&mut rng);
        if posicion % 2 == 1 {
            return posicion;
        }
    }
}
