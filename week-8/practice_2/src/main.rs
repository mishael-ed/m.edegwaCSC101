fn main() {
    let v = vec!['c', 'O', 'P'];

    println!("Enter a number btn 0-2");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Couldn't read line");
    let index:usize = input1.trim().parse().expect("Invalid number");

    let carr:char = v[index];
    println!("Your character is - {}", carr);

}
