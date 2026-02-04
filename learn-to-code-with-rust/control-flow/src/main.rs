fn main() {
    let mut i = 21;

    while i > 0 {
        if i % 2 == 0 {
            println!("{i} é par, adicionando mais 3");
            i -= 3;
            continue;
        }

        println!("{i} ola mundo!");

        i -= 1;
    }

    println!("Tchau mundo");
}
