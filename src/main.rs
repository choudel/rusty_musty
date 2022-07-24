struct Rectangle{
    width: u32,
    height:u32
}
impl Rectangle {
    fn print_description(&self){
        println!("rectangle: {} * {}",self.height,self.width)
    }
    fn is_sqr(&self)->bool{
        self.height==self.width
    }
}
fn main() {
  let my_rect=Rectangle{width:10, height:5};
  my_rect.print_description(); 
  println!("the rectangle is square:{}",my_rect.is_sqr());
}
