fn main() {
    let comida = "📦";
    println!("{}", comida.len());

    let pedaco_pizza = &comida[0..3];
    println!("{}", pedaco_pizza.len());
}
