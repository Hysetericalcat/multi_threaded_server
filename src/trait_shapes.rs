trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    length: f64,
    breadth: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.breadth
    }
}

fn print_area<T: Area>(shape: &T) {
    println!("Area: {}", shape.area());
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle { length: 4.0, breadth: 3.0 };
    print_area(&c);
    print_area(&r);
}