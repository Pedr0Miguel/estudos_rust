fn main() {
    let registros = [true, false, true];

    let first = registros[0];

    println!("{first} e array {registros:?}");
    
    let langs = [String::from("Rust"), String::from("Go")];
    let first_lang = &langs[0];
    println!("{first_lang} e array {langs:?}");
}
