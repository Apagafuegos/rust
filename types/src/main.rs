fn main() {
    let lista_numeros = vec![33, 14, 54, 55, 44];
    let resultado = mayor(&lista_numeros);

    println!("El numero más grande es {resultado}");

    let lista_numeros = vec![0, 14, 3, 5, 6, 7];
    let resultado = mayor(&lista_numeros);

    println!("El numero más grande es {resultado}");
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
