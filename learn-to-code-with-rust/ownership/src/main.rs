fn main() {

    let mut comida_atual = String::new();
    comida_atual = add_sabor(comida_atual);
}


fn add_sabor(mut comida: String)-> String{
    comida.push_str(" sabor energético");
    comida
}