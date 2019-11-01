extern crate mymath;

use mymath::calci;

fn main() {
    let result_add: i32 = calci::add(3,4);

    println!("Add result: {} ", result_add);
    println!("div :{} ",calci::div(10,2));
    println!("modulus:{} ",calci::modulus(8,3));
}