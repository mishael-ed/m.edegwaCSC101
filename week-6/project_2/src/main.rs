use std::io;

fn main() {
    println!("How many siblings do you have?");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Cannot read line");
    let mut siblingnumber:i32 = input1.trim().parse().expect("Not an age");
    
    for name in siblingnumber {
        println!("Enter name of sibling");
    }
}
