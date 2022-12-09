use std::io;

fn main() {
    println!("Using the corresponding numbers, select a computation of your choice");
    println!("\n ---CHOOSE---");
    println!("1 - Area of Trapezium");    
    println!("2 - Area of Rhombus"); 
    println!("3 - Area of Parallelogram"); 
    println!("4 - Area of Cube"); 
    println!("5 - Area of Cylinder"); 

    let mut input1 = String::new();
    
    io::stdin().read_line(&mut input1).expect("Could not read input");
    let fo:i32 = input1.trim().parse().expect("Could not read number");

    if fo == 1 {
        trapezium();
    }

    if fo == 2 {
        rhombus();
    }

    if fo == 3 {
        parallelogram();
    }

    if fo == 4 {
        cube();
    }

    if fo == 5 {
        cylinder();
    }
}

fn trapezium() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nEnter Base 1:");
    io::stdin().read_line(&mut input1).expect("Could not read input");
    let base1:i32 = input1.trim().parse().expect("Invalid Base");

    println!("\nEnter Base 2:");
    io::stdin().read_line(&mut input2).expect("Could not read input");
    let base2:i32 = input2.trim().parse().expect("Invalid Base");

    println!("\nEnter Height:");
    io::stdin().read_line(&mut input3).expect("Could not read input");
    let height:i32 = input3.trim().parse().expect("Invalid Base");

    let area = height * (base1 + base2)/2;
    println!("Area of Trapezium = {}", area);
}

fn rhombus() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    

    println!("\nEnter Diagonal 1:");
    io::stdin().read_line(&mut input1).expect("Could not read input");
    let diagonal1:i32 = input1.trim().parse().expect("Invalid Base");

    println!("\nEnter Diagonal 2:");
    io::stdin().read_line(&mut input2).expect("Could not read input");
    let diagonal2:i32 = input2.trim().parse().expect("Invalid Base");

    let area = (diagonal1 * diagonal2)/2;
    println!("Area of Rhombus = {}", area);
}

fn parallelogram() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    

    println!("\nEnter Base: ");
    io::stdin().read_line(&mut input1).expect("Could not read input");
    let base:i32 = input1.trim().parse().expect("Invalid Base");

    println!("\nEnter Altitude 2:");
    io::stdin().read_line(&mut input2).expect("Could not read input");
    let altitude:i32 = input2.trim().parse().expect("Invalid Base");

    let area = base * altitude;
    println!("Area of Rhombus = {}", area);
}

fn cube() {
    let mut input1 = String::new();
    

    println!("\nEnter length of the side: ");
    io::stdin().read_line(&mut input1).expect("Could not read input");
    let length:i32 = input1.trim().parse().expect("Invalid Base");

    let area = 6 * (length * length);
    println!("Area of Cube = {}", area);
}

fn cylinder() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    

    println!("\nEnter Radius: ");
    io::stdin().read_line(&mut input1).expect("Could not read input");
    let radius:f32 = input1.trim().parse().expect("Invalid Base");

    println!("\nEnter Height: ");
    io::stdin().read_line(&mut input2).expect("Could not read input");
    let height:f32 = input2.trim().parse().expect("Invalid Base");

    let area:f32 = 3.14159 * (radius * radius) * height;
    println!("Volume of Cylinder = {}", area);
}