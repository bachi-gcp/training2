pub mod calci {
    pub fn sub(a: i32, b: i32) ->i32 {
        a - b
    }
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    pub fn print(){
     println!("binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);
    }
    use std::mem;
fn analyze_slice(slice:&[i32])
{
    println!("first element of slice :{}",slice[0]);
    println!("the slice {} elements ",slice.len());
}
fn main()
{
let xs:[i32;5]=[1,2,3,4,5];
let ys:[i32;500]=[0;500];
println!("first element of array:{}",xs[0]);
println!("second element of array:{}",xs[1]);
println!("array size:{}",xs.len());
println!("array occupies {} bytes",mem::size_of_val(&xs));
println!("borror the whole array as a slice");
analyze_slice(&xs);
println!("borrow a section of array as a slice");
analyze_slice(&ys[1 .. 4]);
}
~
~
~
~               


}
