fn main() {
    let month_days: std::ops::Range<i32> = 1..26;

    println!("{month_days:?}");

    let month_days = 1..=26;
    println!("{month_days:?}");

    for day in month_days {
        println!("{day}");
    }

    let letras: std::ops::Range<char> = 'b'..'p';

    for letra in letras {
        println!("{letra}");
    }
}
