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

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    for n in 0.. months.len(){
        println!("{}", months[n]);
    }

    



}
