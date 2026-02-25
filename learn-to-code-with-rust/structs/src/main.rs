struct Coffe {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    let name = String::from("Mocha");

    let coffe: Coffe = make_coffe(name, 3.99, true);

    println!("{} {} {}", coffe.name, coffe.price, coffe.is_hot)
}

fn make_coffe(name: String, price: f64, is_hot: bool) -> Coffe {
    Coffe {
        name: name,
        price: price,
        is_hot: is_hot,
    }
}
