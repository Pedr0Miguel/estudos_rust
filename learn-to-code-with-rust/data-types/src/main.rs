fn main() {
    let first_initial = 'b';
    let emoji = '🙂';

    println!(
        "{}, {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );

    println!("{}, {}", first_initial.is_uppercase(), emoji.is_lowercase());

    println!("{}, {}", first_initial.is_uppercase(), emoji.is_lowercase());
}
