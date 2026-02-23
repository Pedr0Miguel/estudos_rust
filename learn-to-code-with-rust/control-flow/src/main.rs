<<<<<<< HEAD
fn main() {
    let number: i32 = 8;

    match number {
        x if x % 2 == 0 => print!("PAR"),
        x if x % 2 != 0 => print!("IMPAR"),
        _ => unreachable!(),
    };
}
=======
fn color_to_number(color: &str) -> i32 {
    if color == "red" {
        return 1;
    } else if color == "green" {
        return 2;
    } else if color == "blue" {
        return 3;
    } else {
        return 0;
    }
}

fn color_to_number_match(color: &str) -> i32 {
    return match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    };
}

fn factorial(mut number: i32) -> i32 {
    let mut multiplicador: i32 = number - 1;
    loop {
        if multiplicador == 0 {
            return number;
        }

        number *= multiplicador;
        multiplicador -= 1;
    }
}
fn factorial_recursive(number: i32) -> i32 {
    if number == 1 {
        return number;
    }

    number * factorial_recursive(number - 1)
}

fn main() {
    println!("{}", color_to_number("red"));
    println!("{}", color_to_number_match("yellow"));

    println!("{}", factorial(5));
    println!("{}", factorial_recursive(4));
}
>>>>>>> refs/remotes/origin/main
