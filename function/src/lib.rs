
pub fn another_function(k: u32) {
    println!("The value of k is : {k}");
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}
 
pub fn cel_to_fah(num: i32) -> i32 {
    let result: i32 = (num *(9/5)) + 32;

    result
}
 
pub fn fibonnaci(n: i32) -> i32 {

    let mut a = 0;
    let mut b = 1;

    if n < 0 {
        a
    } else if n == 1 {
        b 
    }else {
        for _elem in 2..n+1 {
            let c=  a + b;
            a = b;
            b = c;
        }

        b
    }
}

pub fn print_difference(x: f32, y:f32){
    println!("The difference between {} and {} is {}", x, y, (x-y).abs())
}

pub fn print_array(a: [f32; 2]){
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

pub fn print_distance(z: (f32, f32)) {
    // Using z.0 and z.1 is not nearly as nice as using x and y.  Lucky for
    // us, Rust supports destructuring function arguments.  Try replacing "z" in
    // the parameter list above with "(x, y)" and then adjust the function
    // body to use x and y.
    println!(
        "Distance to the origin is {}",
        ( z.0.powf(2.0) + z.1.powf(2.0) ).sqrt());
}