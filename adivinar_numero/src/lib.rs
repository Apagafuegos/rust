pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "El valor debe de ser o mayor de 1 o menor de 100 y me diste {}",
                value
            );
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
