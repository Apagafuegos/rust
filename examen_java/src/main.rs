use rand::Rng;

fn main() {
    ejercicio_1();
}

fn ejercicio_1(){
    //Generar un array de un tamaño entre 10 y 20 (aleatorio e impar), rellenarlo con los numeros del 50 al 1 y en las posiciones 
    //impares poner la suma total de los valores 
    let mut rng = rand::thread_rng();
    let mut salir_gen = true;
    let tamaño = 0;
    while salir_gen {
        let tamaño = rng.gen_range(10..=20);
        if tamaño & 2 == 1{
            salir_gen = false;
        }        
    }
    let mut array_numeros = vec![tamaño];
    let mut contador = 50;

    for i in 0..array_numeros.len(){
        array_numeros[i] = contador;
        contador -= 1;
    }
    println!("Array inicial");
    for i in 0..array_numeros.len(){
        print!("{} ",array_numeros[i]);
    }

    let mut suma = 0;
    for i in 0..array_numeros.len(){
        suma += array_numeros[i];
    }
    println!("La suma es {}",suma);

    for i in 0..array_numeros.len(){
        if i%2==1{
            array_numeros[i] = suma;
        }
    }
    println!("Array modificado");
    for i in 0..array_numeros.len(){
        print!("{} ",array_numeros[i]);
    }

}