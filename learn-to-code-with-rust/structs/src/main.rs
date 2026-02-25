fn main() {
    struct Coffe {
        name: String,
        price: f64,
        hot: bool,
    }

    let mut beverage = Coffe {
        name: String::from("Mocha"),
        price: 1.99,
        hot: false,
    };

    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 2.99;
    beverage.hot = true;

    println!("{}, {}, {},", beverage.name, beverage.price, beverage.hot);

    let nome_cafe = beverage.name;

    println!("{nome_cafe}");
}
