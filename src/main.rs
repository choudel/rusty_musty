use std::collections::HashMap;
fn main() {
    let mut marks = HashMap::new();

    marks.insert("Rust programming", 96);
    marks.insert("Web Development", 94);
    marks.insert("UX Design", 75);
    marks.insert("Professional Computing Studies", 45);

    println!("How many subjects have you stadied? {}", marks.len());
    match marks.get("Web Development") {
        Some(mark) => println!("you got {} for Web Dev!", mark),
        None => println!("You didn't study that LOL"),
    }
    marks.remove("UX Design");
    for (subject, mark) in &marks {
        println!("For {} you got {}%!",subject,mark);
    } 
    println!("did you study C++? {}",marks.contains_key("C++ Programming"));
}
