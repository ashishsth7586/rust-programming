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

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: 10,
        height: 20
    };

    let rect2: Rectangle = Rectangle { width: 9, height: 19 };
    let rect3: Rectangle = Rectangle { width: 11, height: 21 };

    // calling associated functions
    let square = Rectangle::square(10);

    // calling method
    let area_of_rect = rect1.area();

    println!("Rect: {:#?}", rect1);
    println!("Area of Rectangle is: {} square pixels.", area_of_rect);

    println!("Rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("Rect1 can hold rect3: {}", rect1.can_hold(&rect3));

    println!("The square is: {:#?}", square);

    println!("The area of square is: {} square pixels.", square.area());

}




