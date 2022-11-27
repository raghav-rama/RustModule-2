// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print







fn main() {
    let mut colors: Vec<Colors> = Vec::new();
    colors.push(Colors::Red);
    colors.push(Colors::Green);
    colors.push(Colors::Blue);
    for color in colors {
    colors.push(Colors::Yellow);
      print_color(color);
    }
  }
  
  fn print_color(color: Colors) {
    match color {
      Colors::Red => println!("Red"),
      Colors::Green => println!("Green"),
      Colors::Blue => println!("Blue"),
      Colors::Yellow => println!("Yellow"),
    }
  }
  
  enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
  }
 






