trait Shape {
    fn area(&self) -> f64;
}

struct Square {
    side : f64,
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
struct Rectangle {
    height : f64,
    width : f64,
}


impl Shape for Rectangle{
    fn area(&self) -> f64 {
        self.width * self.height
    }
}