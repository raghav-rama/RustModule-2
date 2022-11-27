// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

fn main() {
    let items = GroceryItems {
        quantity: 10,
        id: 2,
    };
    print_quantity(items);
    print_id(items);
}

fn print_quantity(grocery: GroceryItems) {
    println!("{}", grocery.quantity);
}

fn print_id(grocery: GroceryItems) {
    println!("{}", grocery.id);
}

#[derive(Clone, Copy)]
struct GroceryItems {
    quantity: i32,
    id: i32,
}