use std::mem;

const MEANING_OF_LIFE:u8 = 42;

fn main() {
    let mut a:u16 = 500;
    println!("a = {}", a);
    a = 2;
    println!("a size = {}", mem::size_of_val(&a));

    let z:isize = 123;
    println!("z size = {}", mem::size_of_val(&z));
    println!("os = {}", mem::size_of_val(&z) * 8);

    let x:char = 'x';
    println!("x = {}", x);

    let g:bool = 4 >= 4;
    println!("g = {}", g);

    println!("pow = {}", i32::pow(2, 8));
    println!("bitwise = {}", 2 << 10);

    {
        let a:i8 = 1;
        println!("a = {}", a);
    }

    println!("a = {}", a);

    println!("MEANING_OF_LIFE={}", MEANING_OF_LIFE);
}
