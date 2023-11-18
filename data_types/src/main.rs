fn main() {
    let x: i32 = 5;
    let y: i32 = 3;

    print!("{x},{y}");
    println!();

    let sum: i32= x + y;
    let diff: i32 = x - y;
    let prod: i32 = x * y;
    let div: i32= x / y;
    let reminder: i32 = x % y;

    funcion_hola();

    println!("{sum},{diff},{prod},{div},{reminder}");

    let t: bool = true;
    let f: bool = false;
    println!("{t}");
    println!("{f}");

    let heart_eyed_cat: char = '😻';
    println!("{heart_eyed_cat}");

    let tup: (i32, i32, i32) = (14,33,69);

    let numero_falonso: i32 = tup.0;
    let victorias_alonso: i32 = tup.1;
    let yo_con_tu_madre: i32 = tup.2;

    println!("{numero_falonso}");
    println!("{victorias_alonso}");
    println!("{yo_con_tu_madre}");

    let a: [i32; 12] = [1, 2, 3, 4, 5, 6, 7, 8 , 9 , 10 , 11, 12];
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    for n in 0.. months.len(){
        print!("{} ", months[n]);
    }
    println!();

    for element in months{
        print!("{} ", element);
    }

    for n in 0.. a.len(){
        print!("{} ",a[n]);
    }

    let num_x = funcion_con_return(33);
    println!("{}",num_x);

    let condicion = false;
    let number2 = if condicion{33} else {0};

    println!("{}",number2);

    loop_hasta_33();
}

fn funcion_hola(){
    println!("Hola bebe :)")
}

fn funcion_con_return(x:i32) -> i32{
    let x = -x;
    return x;
}

fn loop_hasta_33(){

    let mut contador = 0;

    while contador < 33 {
        println!("{}",contador);
        contador += 1;
    }

}