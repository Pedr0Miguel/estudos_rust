fn main() {

    let burguer = String::from("Burguer");

    add_fries(burguer); // let lanche = burguer;
    // movemos a posse para lanche

    // println!("{burguer}");

}


fn add_fries(mut lanche: String){
    lanche.push_str(" with Fries.");
    println!("{lanche}");
}