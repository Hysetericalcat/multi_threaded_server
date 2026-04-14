pub enum Shapes {
    Circle(f64),
    Rectangle(f64,f64),
    Triangle(f64,f64,f64)
}


//param s and match s should be same
pub fn area(s:Shapes){
    match s {
        Shapes::Circle(r)=>println!("Area of the Circle: {}",3.14*r*r),
        Shapes::Rectangle(l,b)=>println!("Area of the rectangle: {}",l*b),
        Shapes::Triangle(coff,w,h)=>println!("Area of the Triangle: {}",coff*w*h)
    }
}