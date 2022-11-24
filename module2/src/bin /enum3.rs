//Topic:Working with enum
//
//

/* 
1-Define a Enum Phones with some brands of phones, 
2-define an enum Headsets with some brands  , 
3-define a struct Shop having two fields Phones and Headsets, 
4-create instance of Shop and print which phone and headsets you have picked 
*/

fn main() {
    let shop = Shop {
        phones: Phones::Motorola,
        headsets: Headets::SkullCandy,
    };
    dbg!("{:?}", shop);
    
}

#[derive(Debug)]
enum Headsets {
    SkullCandy,
    Boat,
    Mivi,
}

#[derive(Debug)]
enum Phones {
    Apple,
    Samsung,
    Motorola,
    Nokia,
}

#[derive(Debug)]
struct Shop {
    phones: Phones,
    headsets: Headsets,
}