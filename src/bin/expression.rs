// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
    let value = 101;
    let res = if value > 100 {
        true
    } else {
        false
    };
    print(res);
}

fn print(b: boolean) {
    match b {
        true => println!("it's big"),
        false => println!("it's small"),
    }
}