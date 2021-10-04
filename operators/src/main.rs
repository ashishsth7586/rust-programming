// Here using `const` value is not changed
// but memory address of the variable is changed (arbitary)
const CONSTANT_VARIABLE: u8 = 42; // no fixed address, global variables

// However, there might be situations where memory address
// of a variable is required
// but if Mut is used we have to use `unsafe` block.
static mut Z: i32 = 123;

fn operators() {
    // arithmetic

    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a = a + 1;
    a -= 2;

    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{}, cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);
}

fn main() {
    operators();
    println!("{}", CONSTANT_VARIABLE);
    unsafe {
        println!("{}", Z);
    }
}
