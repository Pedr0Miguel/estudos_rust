fn main() {
    let mut coffe = String::from("Cafee");
    let a = &mut coffe;
    let b = a;

    println!("{a}, {b}");
}