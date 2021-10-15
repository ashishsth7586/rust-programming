use std::usize;

fn main() {
    // --- Ownership Rules ---
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // assigning a variable of type int to another variable, by default, value is copied
    // but if type is `String` instead of copy the value is moved and deallocated from the memory.

    {
        // s is not valid here, it's not yet declared.
        let s: &str = "hello fixed size allocation on Stack."; // s is valid from this point forward
        println!("{}", s);

        // s is string literal, string literal are stored in binary and fixed in Size.
        // what if we want to use a dynamic length string. for that we use String Type.
        // So, now the string of type `String` will be stored in Heap
        let dynamic_string: String = String::from("Hello Dynamic String Allocation with heap");
        println!("{}", dynamic_string);
        // do stuff with s
    } // Here scope is now over, and s, dynamic_string is no longer valid
      // - Rust Automatically drops the value. (deallocates the memory on the heap)

    let s1: String = String::from("Hello");
    let s2: String = s1; // By default, Rust moves the value (no shallow copy or clone)
                         // for cloning the value we use a clone method.
    let s3: String = s2.clone();
    // But with Integer, by default value is copied not moved
    let x: i32 = 5;
    let y: i32 = x;

    println!("{} {}", x, y);
    println!("{} {}", s2, s3);

    // Ownership and Functions

    // Here in example below, `s` is a string and `takes_ownership` is a function
    // that takes in some string and print that string.
    // In the example code below, if we try to print variable `s` after calling `takes_ownership` function
    // and passing the variable `s` there, we get error saying: "`s` can not be borrowed after it is moved."
    // That means, when we pass parameters into a function its the same as if we were to assign `s` to another variable
    // so here, passing in `s` moves `s` into `some_string` variable and `some_String` gets printed out
    // after the function scope is done, `some_string` gets dropped.
    // Note: This is not  the case working with Integer.

    let s: String = String::from("hello World Function");
    takes_ownership(s.clone());
    println!("{}", s);

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    // Lastly, we could take ownership and give it back. For example:

    let string1: String = gives_ownership();
    let string2: String = String::from("Hello");
    let string3: String = takes_and_give_back(string2);
    println!("String1: {}, String3 = {}", string1, string3);

    fn gives_ownership() -> String {
        let some_string: String = String::from("hello");
        some_string
    }
    // here in this function, we are moving the value of `string2` into the function
    // and then we're just returning a string which moves the value back out of the function
    // into string3
    // moving ownership into a function and back is tedious -  so thats where `references` come in to place.
    fn takes_and_give_back(a_string: String) -> String {
        a_string
    }

    // References:

    // `s` in caculate_length is a reference to a string.
    // references don't take ownership of the underlying value.
    // passing in the variable references is refered as `borrowing`
    // References are immutable by default
    let string_a: String = String::from("hello");
    let len: usize = calculate_length(&string_a);
    println!("The length of '{}' is {}.", string_a, len);

    fn calculate_length(s: &String) -> usize {
        let length: usize = s.len(); // len() returns the length of a string
        length
    }

    // Mutable References
    // Note:
    // 1. We can only one mutable references to a particular piece of data in a particular scope.
    // 2. We cant have a mutable reference if immutable reference already exists. Since immutable reference dont expect underlying value to change.
    // But we can however have multiple immutable references.
    // 3. References must be valid.
    let mut string_mut: String = String::from("hello");
    change(&mut string_mut);

    fn change(some_string: &mut String) {
        some_string.push_str(", world!")
    }
    println!("{}", string_mut);

    // Slices
    let s: String = String::from("Hello, World");
    let s2: &str = "hello world";
    let word: &str = first_word(&s);
    let word_from_string_literal = first_word(s2);

    fn first_word(s: &str) -> &str {
        let bytes: &[u8] = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        return &s[..];
    }
    println!("{}", word);
    println!("{}", word_from_string_literal);

}