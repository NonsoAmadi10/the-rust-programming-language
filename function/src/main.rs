fn main() {
    println!("Hello, world!");

    another_function(15);

    let x = plus_one(5);

    println!("The value of x is: {x}");

    let fah = cel_to_fah(x);

    let fib = fibonnaci(9);

    println!("The value of x in fahereint is {fah}");

    println!("The value of the fibonnaci is {fib}");
}

fn another_function(k: u32) {
    println!("The value of k is : {k}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
 
fn cel_to_fah(num: i32) -> i32 {
    let result: i32 = (num *(9/5)) + 32;

    result
}
 
fn fibonnaci(n: i32) -> i32 {

    let mut a = 0;
    let mut b = 1;

    if n < 0 {
        a
    } else if n == 1 {
        b 
    }else {
        for elem in 2..n+1 {
            let mut c = a + b;
            a = b;
            b = c;
        }

        b
    }
}