fn main() {
    let car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;

    println!("{} and {} and {}", &car, ref1, ref2);
}