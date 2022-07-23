fn main() {
    let s1:String=String::from("hello");
    let len:usize=calculate_length(&s1);
    println!("the length of {} is {}",s1,len)

}
fn calculate_length(s:&String)->usize{
    let length:usize=s.len();
    length
}
