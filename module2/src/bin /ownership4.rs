
fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("ऋत्विक");
    // Convert String to Vec
    let _s = s.clone().into_bytes();
    s
}
