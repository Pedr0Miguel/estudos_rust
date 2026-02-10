fn main() {
    let my_value = 2;
    let my_address: &i32 = &my_value;

    println!("{}", my_address);
    
    let heap_value = String::from("Toyota");
    let heap_address = &heap_value;

    println!("{}", heap_address);

}
