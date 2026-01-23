fn main() {
    let apples = 50;
    let oranges = 14 + 5;
    let fruit = apples + oranges;

    println!("This year, my garden has {apples} apples and {oranges} oranges.");
    println!(
        "This year, my garden has {} apples and {} oranges.",
        apples, oranges
    );
    println!("Total of fruits {}.", fruit);
}
