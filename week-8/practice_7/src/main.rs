fn main() {
    let mut mountainheights = ("Everest", 9999, "Kilimanjaro", 89457);

    println!("Original tuple = {:?}", mountainheights);

    mountainheights.2 = "Asemo";
    mountainheights.3 = 7594;

    println!("Changed tuple = {:?}", mountainheights);
}
