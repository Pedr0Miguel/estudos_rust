fn main() {
    let primeiro_nome = {
        let actor: &str = "Arnold Schwarzenegger";
        &actor[0..6]
    };

    println!("{primeiro_nome}");
}
