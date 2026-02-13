fn main() {
    let mut car = String::from("Red");
    let ref1 = &mut car;
    
    ref1.push_str(" e preta");

    println!("{}", ref1);

    let ref2 = &car;

    println!("{}", ref2);
}