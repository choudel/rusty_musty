
fn main() {
    let numbers:[i32;4]=[2;4] ;

   
    for n in numbers.iter(){
        println!("the number is {}",n)
    }
    for i in 1..numbers.len(){
        println!("the number iterated by index {}",numbers[i])
    }
    for j in 2..numbers.len(){
        println!("the number iterated index {}",numbers[j])
    }
}
