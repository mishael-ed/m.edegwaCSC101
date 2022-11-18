use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Input a: ");
    io::stdin().read_line(&mut input1).expect("Failed to read String");
    let a:f32 = input1.trim().parse().expect("Failed to input");

    println!("Input b: ");
    io::stdin().read_line(&mut input2).expect("Failed to read String");
    let b:f32 = input2.trim().parse().expect("Failed to input");

    println!("Input c: ");
    io::stdin().read_line(&mut input3).expect("Failed to read String");
    let c:f32 = input2.trim().parse().expect("Failed to input");

    let d = b * b - 4.0 * a * c;

    if d == 0.0 {
        println!("Roots are real and equal");
        let r1 = -b / (2.0 * a);
        println!("Root1 = {}", r1);
        println!("Root2 = {}", r1);
    }

    else if d > 0.0 {
        println!("Roots are real and different");
        let r1 = (-b + d.sqrt()) / (2.0 * a);
        let r2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Root1 = {}  ", r1);
        println!("Root2 = {}  ", r2);
    }
    else if d < 0.0 {
        println!("Imaginary Roots");
        let p = -b / (2.0 * a);
        let d = d.abs();
        let id = d.sqrt() / (2.0 * a);
        println!("Root1 = {}  +i {}", p, id);
        println!("Root2 = {}  -i {}", p, id);
    }
}
  