fn main() {
    //making of tuple with datatype
    let datatype_tuple : (&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tuple contents = {:?}", datatype_tuple);

    //making of tuple without datatype
    let no_datatype_tuple = ("Rust", "fun", 100);
    println!("Tuple contents = {:?}", no_datatype_tuple);

    //accessing tuple elements
    println!("Value at index 0 = {}", datatype_tuple.0);

    println!("Value at index 1 = {}", datatype_tuple.1);

    println!("Value at index 2 = {}", datatype_tuple.2);

}
