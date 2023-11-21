fn main() {
    let usuario1 = User {
        active: true,
        username: String::from("Extinthor17"),
        email: String::from("carletronix@gmail.com"),
        sign_in_count: 69,
    };

    let _usuario2 = User {
        email: String::from("csantos6952@ceuandalucia.es"),
        ..usuario1
    };
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn _define_usuario(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
