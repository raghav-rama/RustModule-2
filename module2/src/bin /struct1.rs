// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces

// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


fn  main() {
    let mut drinks = Vec::new();
    let mojito = Drinks {
    	flavour: Flavours::Sour,
        ounces: 32,
    };
    let mocha = Drinks {
        flavour: Flavours::Sweet,
        ounces: 64,
    };
    drinks.push(mojito);
    drinks.push(mocha);
    for drink in drinks.iter() {
        match drink.flavour {
            Flavours::Sweet => println!("Sweet"),
            Flavours::Sour => println!("Sour"),
            _ => println!("{:?}, new flavour invented by you :')", drink.flavour),
        }
    }


}

#[derive(Debug)]
struct Drinks {
    flavour: Flavours,
    ounces: i32,
}


#[derive(Debug)]
enum Flavours {
    Sweet,
    Sour,
}
