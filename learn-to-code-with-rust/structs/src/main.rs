struct Coffe {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {

    let coffe: Coffe = make_coffe(String::from("Coffe"), 3.99, true);

    let latte: Coffe = make_coffe(String::from("Latter"), ..coffe);

    

}

fn make_coffe(name: String, price: f64, is_hot: bool) -> Coffe {
    Coffe {
        name,
        price,
        is_hot,
    }
}
