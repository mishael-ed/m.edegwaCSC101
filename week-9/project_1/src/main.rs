use std::io::Write;

fn main() {

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("lager - 33 Export, Desperados, Goldberg, Gulder, Heineken, Star\n".as_bytes()).expect("Write failed");
    file.write_all("stout - Legend, Turbo King, Williams\n".as_bytes()).expect("Write failed");
    file.write_all("non_alcoholic - Maltina, Amstel Malta, Malta Gold, Fayrouz".as_bytes()).expect("Write failed");

    print!("WRITE SUCCESSFUL");
}
