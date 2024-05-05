use std::ops::Add;

#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T
}

impl<T> Add for Point<T>
    where T: Add<Output = T> {
        type Output = Self;
    fn add(self, rhs:Self) -> Self {
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


fn main() {

    let coord = Point{ x: 5.0, y: 6.0};
    let coord2 = Point{ x: 1.0, y: 3.0};

    let sum = coord + coord2;

    println!("{:?}", sum);
}
