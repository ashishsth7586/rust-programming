// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let sum = add(5,6);
    let diff = subtract(8, 3);
    println!("The sum is: {:?}", sum);
    println!("The difference is: {:?}", diff)

}
