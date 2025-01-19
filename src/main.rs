trait CalcArea {
    fn calc_area(&self) -> f64;
}

trait CalcLength {
    fn calc_length(&self) -> f64;
}

struct Line {
    length: f64,
}

impl CalcLength for Line {
    fn calc_length(&self) -> f64 {
        self.length
    }
}

impl CalcLength for Rectangle {
    fn calc_length(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl CalcLength for RightTriangle {
    fn calc_length(&self) -> f64 {
        self.base + self.height + (self.base.powi(2) + self.height.powi(2)).sqrt()
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

struct RightTriangle {
    base: f64,
    height: f64,
}

impl CalcArea for RightTriangle {
    fn calc_area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

fn area<T: CalcArea>(shape: &T) -> f64 {
    shape.calc_area()
}

fn length<T: CalcLength>(shape: &T) -> f64 {
    shape.calc_length()
}

fn main() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    println!("Area of rectangle: {}", area(&rectangle));
    println!("Length of rectangle: {}", length(&rectangle));
    let triangle = RightTriangle {
        base: 3.0,
        height: 4.0,
    };
    println!("Area of triangle: {}", area(&triangle));
    println!("Length of triangle: {}", length(&triangle));

    let line = Line { length: 5.0 };
    println!("Length of line: {}", length(&line));
}
