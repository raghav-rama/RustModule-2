// Topic: Getting Used to with struct
//
//Requirements:
/*
 1-Define  struct Shape having  square:Square and rectangle:Rectangle 
 2-Square has one field as side:i32
 3-Rectangle has two length:i32 and width:i32
 4-Create a function which returns a new Rectangle
 5-Create a function which returns a new Square
 6-Create a function which returns a new Shape 
 7-Create a function which takes Shape and prints the dimension of all shapes 
*/

use std::io;

struct Shape {
    square: Square,
    rectangle: Rectangle,
}

struct Square {
    side: i32,
}

struct Rectangle {
    length: i32,
    width: i32,
}

fn sq(side: i32) -> Square {
    Square { side }
}

fn rect(length: i32, width: i32) -> Rectangle {
    Rectangle { length, width }
}

fn shape(square: Square, rectangle: Rectangle) -> Shape {
    Shape { square, rectangle }
}

fn print(shape: Shape) {
    println!("Side of Square: {}", shape.square.side);
    println!("Length of Rectangle: {}", shape.rectangle.length);
    println!("Widht of Rectangle: {}", shape.rectangle.width);
}

fn main() {
    let side;
    let length;
    let width;

    let mut buf = String::new();

    println!("Enter side of square:");
    io::stdin()
    .read_line(&mut buf)
    .expect("Problem reading stdin");

    side = buf
    .trim()
    .parse::<i32>()
    .unwrap();

    println!("Enter length of rectangle: ");
    buf = String::new();
    io::stdin()
    .read_line(&mut buf)
    .expect("Problem reading stdin");

    length = buf
    .trim()
    .parse::<i32>()
    .unwrap();
    
    println!("Enter width of rectangle: ");
    buf = String::new();
    io::stdin()
    .read_line(&mut buf)
    .expect("Problem reading stdin");
    
    width = buf
    .trim()
    .parse::<i32>()
    .unwrap();

    let sq = sq(side);
    let rect =  rect(length, width);
    let shape = shape(sq, rect);
    print(shape);
}