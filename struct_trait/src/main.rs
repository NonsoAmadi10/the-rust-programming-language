// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
//  trait Bite...

trait Bite {
    fn bite(self: &mut Self);
}

struct Car<'a> {
    mpg: &'a str, // Immutable reference to MPG
    color: &'a str,
    top_speed: i32,
}

impl<'a> Car<'a> {
    fn set_mpg(&mut self, value: &'a str){
        self.mpg = value;
    }

    fn set_color(&mut self, color: &'a str){
        self.color = color;
    }

    fn set_top_speed(&mut self, speed: i32){
        self.top_speed = speed;
    }
}


// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//
#[derive(Debug)] // include this line right before your struct definition
// struct Grapes...

struct Grapes {
    remainder: u32,
}


// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.
//
// impl Bite for...

impl Bite for Grapes{
    fn bite(self: &mut Self) {
        self.remainder -=1 
    }
}


fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    // 4. Uncomment and adjust the code below to match how you defined your
    // Grapes struct.
    //
    let mut grapes = Grapes { remainder: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //
    

    fn bunny_nibbles<T: Bite> (item: &mut T){
        for _ in 0..5{
            item.bite();
        }
    }

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);

    let stadium = Square{ height: 5, width: 10};
    println!("{:?}", stadium.area());


    let mut toyota = Car{ color: "red", mpg: "camry", top_speed: 20};

    toyota.set_color("green");
    toyota.set_top_speed(500);
    toyota.set_mpg("corolla");

    println!("the car has a top speed of {} and it is {} in color with mpg {}", toyota.top_speed, toyota.color, toyota.mpg);


}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

struct Square {
    height: u32,
    width: u32,
}

impl Square{
    fn area(&self)->u32{
        self.height * self.width
    }
}


