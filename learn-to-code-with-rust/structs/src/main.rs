struct Coffe {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    let mut coffe: Coffe = make_coffe(String::from("Coffe"), 3.99, true);

    drink_coffe(&mut coffe);

    print!("{}", coffe.price);
}

fn make_coffe(name: String, price: f64, is_hot: bool) -> Coffe {
    Coffe {
        name,
        price,
        is_hot,
    }
}

fn drink_coffe(coffe: &mut Coffe) {
    println!("Drinking my {}", coffe.name);
    coffe.price = 10.99;
    coffe.is_hot = false;
}
