fn salvou_dia(nome: &str) {
    println!("{nome} salvou o dia");
}

fn main() {
    let actor = String::from("Arnold Schwarzenegger");
    salvou_dia(&actor);
    let outro = "Syvester Stallone";
    salvou_dia(&outro);

    let primeiro_nome = &actor[0..6];
    salvou_dia(primeiro_nome);
}
