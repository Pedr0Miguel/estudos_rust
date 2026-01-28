/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.

Cast the i32 to an i16 integer and assign the result
to a separate variable.

Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.

Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.

Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.

Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.

Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.

Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/

fn main() {
    let variable_1: i32 = 1_337;

    let variable_i16: i16 = variable_1 as i16;

    let float_number: f64 = 2412.5564534;

    println!("{float_number:.3}");

    let with_milk = false;
    let with_sugar = false;

    let is_my_type_of_coffee = with_milk && with_sugar;

    let is_acceptable_coffee = with_milk || with_sugar;

    let array_of_i8: [i8; 6] = [1, 2, 3, 4, 5, 6];

    dbg!(array_of_i8);

    let tupla_everything = (variable_1, float_number, with_milk, array_of_i8);

    dbg!(tupla_everything);
}
