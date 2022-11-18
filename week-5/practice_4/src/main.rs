use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Please enter your name: ");
    io::stdin().read_line(&mut input1).expect("Failed to read string");
    let name = input1;

    println!("\nPlease enter your age: ");
    io::stdin().read_line(&mut input2).expect("Failed to read string");
    let age:i32 = input2.trim().parse().expect("Failed to read Number");

    if age >= 18 {
        println!("Welcome to the party {}!", name);
    } else {
        println!("Oops, your are not of age to enter the party, {}!", name);
    }
}