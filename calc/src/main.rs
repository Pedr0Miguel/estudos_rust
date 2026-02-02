use std::io;

fn soma(number1: i32, number2: i32) -> i32 {
    number1 + number2
}

fn subtrair(number1: i32, number2: i32) -> i32 {
    number1 - number2
}

fn divisao(number1: f32, number2: f32) -> f32 {
    number1 / number2
}

fn multiplicacao(number1: i32, number2: i32) -> i32 {
    number1 * number2
}

fn calculadora() {
    loop {
        println!("Selecione uma opção:");
        println!("1 - Soma\n2 - Subtrair\n3 - Divisão\n4 - Multiplicação\n5 - SAIR");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: u32 = input.trim().parse().unwrap();

        if input == 5 {
            println!("Bye :p");
            break;
        }

        println!("Digite o primeiro numero:");
        let mut numero = String::new();
        io::stdin().read_line(&mut numero).unwrap();
        let n1: f32 = numero.trim().parse().unwrap();

        numero.clear();

        println!("Digite o segundo numero:");
        io::stdin().read_line(&mut numero).unwrap();
        let n2: f32 = numero.trim().parse().unwrap();

        match input {
            1 => println!("Resultado: {}", soma(n1 as i32, n2 as i32)),
            2 => println!("Resultado: {}", subtrair(n1 as i32, n2 as i32)),
            3 => println!("Resultado: {}", divisao(n1, n2)),
            4 => println!("Resultado: {}", multiplicacao(n1 as i32, n2 as i32)),
            _ => println!("Opção inválida"),
        }
    }
}

fn main() {
    calculadora();
}
