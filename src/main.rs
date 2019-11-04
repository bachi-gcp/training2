extern crate mymath;

use mymath::calci;
mod temp;

fn main() {
    let result_add: i32 = calci::add(3,4);

    println!("Add result: {}", result_add);
    println!("mul result: {}", calci::mul(3,4));
    println!("mul result: {}", calci::equal(4,4));
    temp::fun();
    
}