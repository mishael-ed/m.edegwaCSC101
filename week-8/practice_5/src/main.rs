fn main() {
    let mut city : Vec<String> = Vec::new();

    println!("The city vectorn has element {}", city.len());

    let mut input1 = String::new();
    println!("How many cities do you want to enter?");
    std::io::stdin().read_line(&mut input1).expect("Could not read line");
    let citynum:i32 = input1.trim().parse().expect("Invalid number");

    for count in 0..citynum {
        let mut input2 = String::new();
        println!("Enter city {}", count+1);
        std::io::stdin().read_line(&mut input2).expect("Could not read line");
        let new_city:String = input2.trim().parse().expect("Invalid input");
        city.push(new_city);
    }

    println!("Your preferred cities are:\n");
    let mut count=1;
    for i in city {
        println!("{}, {}", count, i);
        count+=1;
    }
}
