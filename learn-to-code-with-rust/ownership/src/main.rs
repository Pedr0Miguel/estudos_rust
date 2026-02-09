fn main() {
    let person: String = String::from("Pedro");

    drop(person);

    let genius= person;
}
