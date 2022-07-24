fn main() {
    let animals = vec!["Rabbit","hnech","klebs"];
    for (index,a) in animals.iter().enumerate(){
        println!("the index is : {} and the animal is {}",index,a);
    }
}
