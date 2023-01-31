use std::io;
use std::io::Read;

fn main() {
    println!("ENTER YOUR IDENTITY BELOW USING THE CORRESPONDING DIGITS");
    println!("\nADMINISTRATOR - 1\nPROJECT MANAGER - 2\nEMPLOYEE - 3\nCUSTOMER - 4\nVendor - 5");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let ans:i64 = input1.trim().parse().expect("Failed to read");

    if ans == 1 {
        admin();
    }
    else if ans == 2 {
        pmanager();
    }
    else if ans == 3 {
        employee();
    }
    else if ans == 4 {
        customer();
    }
    else if ans == 5 {
        vendor();
    }
    else {
        println!("Position not recognized");
    }
}

fn admin() {
    let mut file = std::fs::File::open("db_struct.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn pmanager() {
    let mut file = std::fs::File::open("project_table.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn employee() {
    let mut file = std::fs::File::open("employee_table.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn customer() {
    let mut file = std::fs::File::open("customer_table.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}

fn vendor() {
    let mut file = std::fs::File::open("dataplan.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}