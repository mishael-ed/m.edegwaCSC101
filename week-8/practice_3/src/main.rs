fn main() {
    let v = vec!['c', 'O', 'P'];

    println!("Enter a number btn 0-2");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Couldn't read line");
    let index:usize = input1.trim().parse().expect("Invalid number");

    let ch: Option<&char> = v.get(index);
    value(ch);
}

fn value(n:Option<&char>) {
    println!("The element is - {:?}", n);
}
