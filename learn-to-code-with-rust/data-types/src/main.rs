fn main() {
    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let marcas: [&str; 3] = ["apple", "Samsung", "Motorola"];

    println!("{}", marcas.len());

    println!("{:?}", marcas);
    // OU
    println!("{numbers:?}");
    println!("{numbers:#?}");
}
