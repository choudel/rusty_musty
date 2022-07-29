struct Object {
    width: u32,
    height: u32,
}
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn new (width:u32, height:u32)->Object{
        Object { width, height }
    }
    fn show(&self) {
        println!("{}and{} the area is: {}", self.width, self.height, self.area());
    }
}

fn main() {
    
    let obj = Object::new(15, 60);
    
    obj.show();
}
