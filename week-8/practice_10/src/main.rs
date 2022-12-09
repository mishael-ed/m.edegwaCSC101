fn main() {
    let numbers = [1, 2, 3, 4, 5];
    println!("Original Array = {:?}", numbers);

    //slice of second and third element
    let slice1 = &numbers[1..3];
    println!("Second and third elements sliced = {:?}", slice1);

    //omit the start index
    let slice2 = &numbers[..3];
    println!("index 0 to index 3 sliced = {:?}", slice2);

    //omit the end index
    let slice3 = &numbers[2..];
    println!("index 2 to index 5 sliced = {:?}", slice3);

    //omit the start index and end index
    let slice4 = &numbers[..];
    println!("index 0 to index 5 sliced = {:?}", slice4);
}
