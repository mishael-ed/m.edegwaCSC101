use std::io;

fn main() {
    let mut name = String::new();
    let mut input2 = String::new();

    println!("Enter your name");
    io::stdin().read_line(&mut name).expect("Couldn't read string");
    
    println!("Enter your age");
    io::stdin().read_line(&mut input2).expect("Couldn't read string");
    let age:i32 = input2.trim().parse().expect("Couldn't read number");

    if age >= 40 {
        println!("The incentive of {} is N1,560,000", name);
    }
    else if age <= 39 && age > 29 {
        println!("The incentive of {} is N1,480,000", name);
    }
    else if age <= 29 {
        println!("The incentive of {} is N1,300,000", name);
    }


}
