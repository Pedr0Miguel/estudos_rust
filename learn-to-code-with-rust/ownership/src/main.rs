fn main() {

    let mut trip = start_trip();

    visit_philadelphia(&mut trip);

    trip.push_str(" and ");

    visit_new_york(&mut trip);

    trip.push_str(" and ");

    visit_boston(&mut trip);

    show_itinerary(&trip);

}

fn start_trip()-> String{
    String::from("The plan is...")
}

fn visit_philadelphia(plan: &mut String){
    plan.push_str("Philadephia")
}

fn visit_new_york(plan: &mut String){
    plan.push_str("New York")
}

fn visit_boston(plan: &mut String){
    plan.push_str("Boston")
}

fn show_itinerary(plan: &String){
    println!("{plan}");
}