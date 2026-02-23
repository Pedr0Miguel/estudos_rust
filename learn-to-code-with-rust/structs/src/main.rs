fn main() {
    struct Coffe {
        name: String,
        price: f64,
        hot: bool,
    }

    let mocha = Coffe {
        name: String::from("Mocha"),
        price: 1.99,
        hot: false,
    };

    println!("{}, {}, {},", mocha.name, mocha.price, mocha.hot);

    let nome_cafe = mocha.name;

    println!("{nome_cafe}");
    println!("{}", mocha.name);
}
