fn main() {
    let number: i32 = 8;

    match number {
        x if x % 2 == 0 => print!("PAR"),
        x if x % 2 != 0 => print!("IMPAR"),
        _ => unreachable!(),
    };
}
