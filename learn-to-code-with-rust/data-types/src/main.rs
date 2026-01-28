fn main() {
    let month_days = 1..26;

    println!("{month_days:?}");

    let month_days = 1..=26;
    println!("{month_days:?}");

    for day in month_days {
        println!("{day}");
    }

    let letras = 'b'..'p';

    for letra in letras {
        println!("{letra}");
    }
}
