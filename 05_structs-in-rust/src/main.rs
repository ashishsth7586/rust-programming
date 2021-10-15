/*
    A struct, or structure, is a custom data type that lets you name and
    package together multiple related values that make up a meaningful 
    group. If you're familiar with an object-oriented language, a struct
    is like an object's data attributes.

    Structs are similar to tuples. Like tuples, the pieces of a struck can
    be different types. Unlike with tuples, you'll name each piece of data
    so it's clear what the values mean. As a result of these names, structs
    are more flexible than tuples: you don't have to rely on the order of
    the data to specify or access the values of an instance.
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect: Rectangle = Rectangle {
        width: 10,
        height: 20
    };

    let area_of_rect = area(&rect);

    println!("Rect: {:#?}", rect);
    println!("Area of Rectangle with Width: {} and Height: {}  is: {}", rect.width, rect.height, area_of_rect);
}


// Here we pass reference to Struct Rectangle
// We want to use its fields but not take ownership.
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

