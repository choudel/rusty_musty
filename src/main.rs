#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}
impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}
fn main(){
    let mut people =vec![
        Person::new("Al".to_string(),25),
        Person::new("John".to_string(),3),
        Person::new("Zoe".to_string(), 23)
    ];
    people.sort_by(|a,b| b.age.cmp(&a.age));
    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(),25),
            Person::new("Zoe".to_string(), 23),
            Person::new("John".to_string(),3),
        ]
    );
    println!("Success");
}