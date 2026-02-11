fn main() {
    let is_concert: bool = true;
    let is_event: bool = is_concert;

    println!("{is_concert}, {is_event}");


    let dinner = "Salmon";
    let sushi = dinner;

    println!("{dinner}, {sushi}");

    let s_ex = String::from("Salmon");
    let mut ex2 = s_ex;

//    println!("{s_ex}, {ex2}");

    ex2 = eat_meal(ex2);

    println!("{ex2}");

}

fn eat_meal(mut meal: String)-> String{
    meal.clear();
    meal
}