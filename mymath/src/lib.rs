
pub mod calci {
    pub fn sub(a: i32, b: i32) ->i32 {
        a - b
    }
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }    
    pub fn mul(a: i32,b: i32) -> i32 {
        a * b
    }
    pub fn equal(a:i32,b:i32) -> bool {
        a == b
    }
    pub fn print() {
        println!("binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);
    }
}
