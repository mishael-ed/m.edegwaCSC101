use std::io;

fn main() {
    println!("--- MENU ---");
    println!("1 = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("2 = Fried Rice & Chicken - N3,000");
    println!("3 = Amala & Ewedu Soup - N2,500");
    println!("4 = Eba & Egusi Soup - N2,000");
    println!("5 = White Rice & Stew - N2,500");

    println!("\nPlease Enter Your Order Using The Numbers Shown Above");
    
    let mut foo = String::new();
    io::stdin().read_line(&mut foo).expect("Item not on the menu.");
    let food:i32 = foo.trim().parse().expect("Could not detect the number");
    
    println!("\nPlease Enter Quantity");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Could not read quantity");
    let q:i32 = quantity.trim().parse().expect("Could not detect number");


    if food == 1 {
        let price = 3200;
        let newquant = price * q;
        if newquant > 10000 {
            let _actp = (newquant * 5)/100;
            println!("Your bill is {} (After 5% Discount)", _actp);
        }
        else {
            println!("Your bill is {}", newquant);     
        }
    }

    else if food == 2 {
        let price = 3000;
        let newquant = price * q;
        if newquant > 10000 {
            let _actp = (newquant * 5)/100;
            println!("Your bill is {} (After 5% Discount)", _actp);
        }
        else {
            println!("Your bill is {}", newquant);     
        }
    }
    else if food == 3 {
        let price = 2500;
        let newquant = price * q;
        if newquant > 10000 {
            let _actp = (newquant * 5)/100;
            println!("Your bill is {} (After 5% Discount)", _actp);
        }
        else {
            println!("Your bill is {}", newquant);     
        }
    }
    else if food == 4 {
        let price = 2000;
        let newquant = price * q;
        if newquant > 10000 {
            let _actp = (newquant * 5)/100;
            println!("Your) bill is {} (After 5% Discount)", _actp);
        }
        else {
            println!("Your bill is {}", newquant);     
        }
    }
    else if food == 5 {
        let price = 2500;
        let newquant = price * q;
        if newquant > 10000 {
            let _actp = (newquant * 5)/100;
            println!("Your bill is {} (After 5% Discount)", _actp);
        }
        else {
            println!("Your bill is {}", newquant);     
        }
    }
}
