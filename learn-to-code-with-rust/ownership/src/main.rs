fn main() {
    let mut name = String::from("Pedro");
    println!("NOME: {name}");

    name.push_str(" Miguel");

    println!("NOME ALTERADO: {name}");

    name.push_str(" Plaça Lima");

    println!("NOME ALTERADO: {name}");
}
