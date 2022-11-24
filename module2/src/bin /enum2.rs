// Use a enum with variants as different car names and
//  create a function which accepts an &str, 
// using match statement return the correct enum variant
// define a struct Details 
// car:Car
//price:i32




use std::io;

fn main() {
    let mut car_name = "";
    print!("Enter Car Name: ");
    io::stdin().read_line(&mut car_name).expect("can't read input");
    println!("{:?}",variant(car_name));
    
}

fn variant(car: &str) -> Car {
    match cars {
        "Bugatti" => Cars::Bugatti,
        "Mercedes" => Cars::Mercedes,
        "Nissan" => Cars::Nissan,
        "Pagani" => Cars::Pagani,
        "Lamborghini" => Cars::Lamborghini
    }
}

enum Car {
    Bugatti,
    Mercedes,
    Nissan,
    Pagani,
    Lamborghini,
}

struct Detail {
    car: Car,
    price: i32,
}


