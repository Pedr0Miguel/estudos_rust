fn main() {
    let mut comida_atual = String::new();
    add_sabor(&mut comida_atual);
}

/**
 * Existe 4 maneiras de commo nós definirmos um parâmetro
 * 
 * comida: String - nome do parâmetro, seu tipo,
 * ele recebe a responsabilidade do valor e que ele é imutável;
 * 
 * mut comida: String - nome do parâmetro, seu tipo,
 * ele recebe a responsabilidade do valor e que ele é mutável;
 * 
 * comida: &String - nome do parâmetro,
 * seu tipo que não é uma string e sim a referência de uma string,
 * ele recebe a referência desse valor mas não altera o valor da memória;
 * 
 * comida: &mut String - nome do parâmetro,
 * seu tipo que não é uma string e sim a referência de uma string,
 * ele recebe a referência desse valor e consegue alterá-lo da maneira que quiser;
 * 
 */

fn add_sabor(comida: &mut String) {
    comida.push_str(" sabor energético");
}

fn mostrar_comida(comida: &String){
    println!("{comida}");
}