fn main() {
    let oranges = String::from("Oranges");

    print_value(oranges); // let value = oranges; Oranges passa a posse de Oranges para value o parâmetro

    println!("oranges {oranges}"); 


}

fn print_value(value: String){
    println!("The value is {value}");
}// Aqui a string Oranges é apagada pois foje do escopo fazendo