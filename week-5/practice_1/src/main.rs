use std::io;

fn main() {
    println!("\nStudent Information Management System!");

    //Input your name
    println!("Please enter your name.");
    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Your name is: {}", name);

    //Input your age
    println!("\nPlease enter your age.");
    let mut age = String::new();

    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age:i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is {}", age);
}