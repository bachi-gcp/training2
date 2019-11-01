extern crate mymath;

use mymath::calci;

fn main() {
    let result_add: i32 = calci::add(3,4);

    println!("Add result: {}", result_add);

    let result_sub:i32=calci::sub(3,4);

    println!("sub result : {}", result_sub);
}
