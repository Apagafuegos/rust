fn main() {
    
    println!("{}",add_two(31));

}

pub fn add_two(a: i32) -> i32{
    a + 3
}

pub fn decir_hola(name: &str) -> String{
    format!("Hola, {}", name)
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn suma_dos(){
        assert_ne!(4,add_two(2)); //True si no son iguales
        assert_eq!(4,add_two(2)); //True si son iguales
    }
    #[test]
    fn contiene_nombre(){
        let result = decir_hola("Carlos");
        assert!(result.contains("Pepe"),"No contiene el nombre, la expresión es {}",result)
    }
}