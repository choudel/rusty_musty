use std::fmt;
#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}
//Methods
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn show(&self) {
        println!("{}and{} the area is: {}", self.width, self.height, self.area());
    }
}
impl Object {
    fn new (width:u32, height:u32)->Object{
        Object { width, height }
    }
}
impl fmt::Display for Object{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({},{})and Area: {}",self.height,self.width,self.area())
    }
}

fn main() {
    
    let obj = Object::new(15, 60);
    
    obj.show();
    println!("{}",obj)
}
