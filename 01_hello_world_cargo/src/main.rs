// directives
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::mem;

fn main() {
    let a: u16 = 356; // unsigned 8-bits, [0 - 255]
    println!("Hello, world!");
    println!("a = {}", a);

    // u = unsigned, 0 to 2^N-1
    // i = signed, -2^(N-1) to 2^(N-1)-1
    // by default, all the variables declared in rust are immutable
    let mut b: i8 = 0;
    println!("B = {} before", b);
    b = 20;
    println!("B = {} after", b);

    let c = 123456789; // i32
    println!("c = {}, takes up {} bytes.", c, mem::size_of_val(&c));

    // usize, isize
    // declares a variable of type native to processor
    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS.",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x';
    println!("{} is a char, size = {} bytes.", d, mem::size_of_val(&d));

    let e: f32 = 2.6;
    println!("{}, size = {} bytes.", e, mem::size_of_val(&e));
}
