pub mod lib;
use crate::lib::restar;
use crate::lib::sumar;
fn main() {
    let suma_total = sumar(33, 14);
    println!("{suma_total}");

    let resta_total = restar(14, 33);
    println!("{resta_total}");
}
