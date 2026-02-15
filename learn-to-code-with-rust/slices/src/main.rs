fn main() {
    let mut values = [2, 63, 32, 68, 21, 4];
    let my_slice: &mut [i32] = &mut values[2..4];

    println!("minha parte {my_slice:?}");
    
    my_slice[0] = 100;
    println!("minha parte {my_slice:?}");
    println!("minha array {values:?}");
    
}
