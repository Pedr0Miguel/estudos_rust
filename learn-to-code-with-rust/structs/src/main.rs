struct Coffe {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    let nome = String::from("Mocha");

    let coffe: Coffe = make_coffe(nome, 3.99, true);

    println!("{} {} {}", coffe.name, coffe.price, coffe.is_hot)


    let name = String::from("Lattee");
    let price = 7.98;
    let is_hot = false;

    let latte = Coffe{name,price, is_hot};
}

fn make_coffe(name: String, price: f64, is_hot: bool) -> Coffe {
    Coffe {
        name,
        price,
        is_hot,
    }
}
