struct Person {
    name: String,
    age: u8,
}
struct Child {
    name: String,
    age: u8,
}
trait HasVoiceBox {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}
impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello,my name is {}", self.name);
    }
    fn can_speak(&self) -> bool {
        if self.age > 3 {
            return true;
        }
        return false;
    }
}
impl HasVoiceBox for Child {
    fn speak(&self) {
        println!("I'm a child and my name is {}", self.name)
    }
    fn can_speak(&self) -> bool {
        if self.age > 0 && self.age < 3 {
            return false;
        }
        return true;
    }
}
fn main() {
    let person = Person {
        name: String::from("choudel"),
        age: 41,
    };
    println!("Can flen '{}' speak? {}", person.name, person.can_speak());
    person.speak();
    let baby = Child {
        name: String::from("pitbull"),
        age: 2,
    };
    println!(
        "can this baby talk? : {}, literaly: {}",
        baby.name, baby.can_speak()
    );
    baby.speak()
}
