// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

use function::{print_distance, another_function, print_array,print_difference, fibonnaci, cel_to_fah, plus_one};
fn main() {
    println!("Hello, world!");

    another_function(15);

    let x = plus_one(5);

    println!("The value of x is: {x}");

    let fah = cel_to_fah(x);

    let fib = fibonnaci(9);

    println!("The value of x in fahereint is {fah}");

    println!("The value of the fibonnaci is {fib}");

    let cords: (f32, f32) = (6.3, 15.0);
    print_difference(cords.0, cords.1);
    let series:[f32;2] = [0.7, 0.8];
    print_array(series);
    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    let coordinates:(f32, f32) = (0.7, 0.5);
    print_distance(coordinates);

}

