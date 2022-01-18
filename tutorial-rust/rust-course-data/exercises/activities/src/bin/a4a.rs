// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    // prefer match over else..if when working with
    // a single variable
    // match considers all posibilities
    // use underscore (_) to match "anything else"
    let some_bool = true;
    match some_bool {
        true => println!("Its True"),
        false => println!("Its False")
    }

    let some_int = 3;
    match some_int {
        1 => println!("Its 1"),
        // _ here means everything else than 1
        _ => println!("Its other than 1, its {:?}", some_int)
    }
}
