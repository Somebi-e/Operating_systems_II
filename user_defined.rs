//here were dealing with struct
pub enum Gender{
    Male,
    Female
}
#[derive(Debug) ] //Debug is a trait this implies that a collection like this, stuct has to be declared and there should be an implementation side. it means we hve forced the rust compiler to implement the trait debug for the struct
struct Person{
    name: String,
     age: u8
}

pub fn run() {
    let person = Person {name: String::from("Peter"), age: 27};
    println!("{:?}", person) //we can only print primitives in{} you cant just print a derived type like that 
}