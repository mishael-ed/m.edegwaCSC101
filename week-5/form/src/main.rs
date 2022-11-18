fn main() {
    let a:i32 = 4;
    let b:i32 = -4;
    let c:i32 = 1;

    let v = i32::pow(b, 2);
    let m:i32 = 4 * a * c;
    let d:i32 = v - m;
    println!("{}", d);
}
