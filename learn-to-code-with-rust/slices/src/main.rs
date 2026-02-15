fn main() {
    let actor = String::from("Arnold Schwarzenegger");
    let referencia = &actor[0..=5];
    println!("{referencia}");

    let sobrenome = &actor[7..21];
    println!("{sobrenome}");
}
