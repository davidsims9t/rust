use std::mem;

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
}
