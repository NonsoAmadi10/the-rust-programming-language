use std::vec;

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

    let (val1, val2) = (5, 2);
    let ans = val1 % val2;

    println!("{}", ans);

    let mut vec_arr = vec![2, 4,6,8,10];
    vec_arr.pop();
    vec_arr.push(12);

    println!("{:?}", vec_arr);

    concat_string("nonso");
    control_flow(30);

    let mut v = vec![1, 3, 5,7];

    add_vec(&mut v);
    v.push(15);
    println!("{:?}", v);

    let w = 3;
    println!("{:?}", add_two(w))
}

fn concat_string(s: &str){
    let hello = String::from("hello");

    println!("{}, {}", hello, s)
} 

fn control_flow(i: u32){
    if i == 1 {
        println!("The value is one")
    }else if i > 50{
        println!("The value is greater than 50")
    }else if i < 25 {
        println!("The value is less than 25")
    } else if i > 25 && i < 50 {
        println!("The value is greater than 25 but less than 50")
    }else {
        println!("The value is {:?}", i)
    }
}

fn add_vec(v: &mut Vec<i32>) -> bool{
    let first_item = v[0];

    first_item == 1

}

fn add_two(u: i8)->i8{
    u + 2
}