fn main() {
    
    let mut x = 6;

    println!("The value of x is: {x}");

    x = 5;
    println!("The value of x is: {x}");

    // shadowing allows you to reuse a variable name by assigning it several values

    let x  = 5;
    let x = x + 1;

    println!("The value of x is : {x}");

    // shadowing is different from mutability in the sense that mutability does not allow for the variable
    // datatype to change 

    // let mut spaces = "  ";
    // spaces = spaces.len();

    // println!("spaces is {spaces}")

    // DataTypes

    // Scalar Types: integers, floating point numbers, booleans and characters
    let u:u32 = 12;
    println!("The value of u is {u}");
    let c = 'z';

    println!("The value of c is {c}");

    // Tuples:

    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    let b: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = b.0;

    let six_point_four = b.1;

    let one = b.2;

    println!("These are the values of {five_hundred}, {six_point_four}, {one}");

    let mut missiles = 8;
    let ready = 2;

    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;

    println!("{} missiles left", missiles);


}
