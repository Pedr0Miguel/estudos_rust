fn main() {
    let actor = String::from("Arnold Schwarzenegger");

    let primeiro_nome = &actor[..6];
    println!("{primeiro_nome}");

    let sobrenome = &actor[7..];
    println!("{sobrenome}");

    let nome_completo = &actor[..];
    println!("{nome_completo}");
}
