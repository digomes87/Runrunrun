#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}
pub fn new_person() {
    let mut people = vec![
        Person::new("Zoe", 25),
        Person::new("Al", 55),
        Person::new("Zeee", 23),
    ];

    people.sort();
    println!("Sorted people: {:#?}", people);
}
