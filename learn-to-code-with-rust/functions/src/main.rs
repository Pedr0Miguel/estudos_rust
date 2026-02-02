fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs");
}

fn is_even(number: i32) -> bool {
    if number % 2 == 0 {
        return true;
    }
    return false;
}

fn alphabets(string: &str) -> (bool, bool) {
    return (string.contains('a'), string.contains('z'));
}

fn main() {
    apply_to_jobs(2, "Rust developer");

    println!("{}", is_even(9));
    println!("{}", is_even(12));

    dbg!(alphabets("aardvark"));
    dbg!(alphabets("zoology"));
    dbg!(alphabets("zebra"));
}
