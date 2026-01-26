fn main() {
    let adicao: i32 = 5 + 9;
    let subtracao: i32 = 5 - 9;
    let multiplicacao: i32 = 5 * 9;
    println!("{adicao}");
    println!("{subtracao}");
    println!("{multiplicacao}");

    let floor_divisao = 5 / 3;
    println!("{floor_divisao}"); // isso vai printar 1 pois podemos dividir 5 por 3 apenas uma vez

    let floor_float = 5.00 / 3.00;
    println!("{floor_float}"); // vai retornar o calculo completo 1.6666

    let resto = 8 % 2;
    let resto2 = 9 % 2;

    println!("{resto}"); // vai retornar o resto da divisão de 8 por 2 que seria 0
    println!("{resto2}"); // vai retornar o resto da divisão de 9 por 2 que seria 1
}
