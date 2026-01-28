fn main() {
    let empregado = ("MOLLY", 32, "Marketing");

    let (nome, idade, setor) = empregado;

    // let nome = empregado.0;
    // let idade = empregado.1;
    // let setor = empregado.2;

    println!("{} tem {} e trabalha no {}", nome, idade, setor);

    dbg!(empregado);
}
