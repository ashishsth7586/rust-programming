/*
    Enumerations, also referred to as `enums`. Enums allow us to define a type
    by enumerating its possible variants. First, we'll define and use an enum
    to show how an enum can encode meaning along with data. Next, we'll explore
    a particularly useful enum, called Option, which expresses that a value can
    be either something or nothing. Then we'll look at how pattern matching in 
    the `match` expression makes it easy to run different code for different 
    values of an `enum`. Finally, we'll cover how the `if let` construct is another
    convenient and consise idiom avialble to us to handle enums in the code.
*/

// enum Option<T> {
//     Some(T),
//     None
// }
fn main() {
    let x: i8 = 5;
    // let y: Option<i8> = None;
    let y: Option<i8> = Some(6);
    
    let sum = x + y.unwrap_or(0); // if y is None then default value set to y is 0
    println!("Sum of {} & {:?} is: {}", x, y, sum);
    

    println!("{}", value_in_cents(Coin::Quater(USState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Six: {:?}", six.unwrap_or(0));
    
    
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(USState)
}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California
}

fn value_in_cents(coin: Coin) -> u8 {
    // match expression: allows use to compare a value
    // against a set of patterns. These patterns could be
    // literals, variables, wildcards etc.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State Quater from: {:#?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}