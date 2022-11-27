// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

use std::io;

fn main() {
    let (x, y) = my_tup();
    if y > 5 {
        println!("y coordinate is greater than 5")
    } else if y == 5 {
        println!("y coordinate is equal to 5")
    } else {
        println!("y coordinate is smaller than 5")
    }
}

fn my_tup() -> (i32, i32) {
    let mut my_tup = (0,0);
    let mut buf = String::new();

    println!("Enter x-coordinate: ");
    io::stdin()
    .read_line(&mut buf)
    .expect("Problem reading stdin");

    let x = buf
    .trim()
    .parse::<i32>()
    .unwrap();

    println!("Enter y-coordinate");
    buf = String::new();
    io::stdin()
    .read_line(&mut buf)
    .expect("Problem reading stdin");

    let y = buf
    .trim()
    .parse::<i32>()
    .unwrap();

    my_tup = (x, y);

    return my_tup;
}