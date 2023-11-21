fn main() {
    let s = String::from("viva el Beti weno");
    let _palabra = first_word(&s[0..6]);
    let _palabra = first_word(&s[..]);
    let palabra = first_word(&s);
    //Funciona tanto para slices del String, como para un slice de todo el String completo como para el String en si
    println!("{palabra}");

    let palabra2 = "viva el cordoba jeje";
    let _s2 = first_word(palabra2); //Palabra 2 ya en si es un slice

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &a[2..3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //Si el elemento(en bytes) en la pos i va seguido de un espacio, devuelve la posicion en la que esta i
            return &s[0..i];
        }
    }

    &s[..]
}
