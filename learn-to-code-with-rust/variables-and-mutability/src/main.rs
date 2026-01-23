const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season: &str = "Invincible";

    let _favorite_beverage: &str = "Whiskey";

    #[allow(unused_variables)]
    let favorite_beverage2: &str = "Whiskey";

    #[allow(unused_variables)]
    let mut points_scored: i32 = 28;

    points_scored = 35;

    let event_time: &str = "06:00";

    println!(
        "Season: {season}; points: {points_scored}; event_time: {event_time}; event_time: {TOUCHDOWN_POINTS};"
    );
    println!(
        "Season: {}; points: {}; event_time: {}; TOUCHDOWN_POINTS: {};",
        season, points_scored, event_time, TOUCHDOWN_POINTS
    );

    println!(
        "Season: {0}; points: {1}; event_time: {2};TOUCHDOWN_POINTS: {3};",
        season, points_scored, event_time, TOUCHDOWN_POINTS
    );
}
