struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Employee<'a> {
    id: &'a str,
    person: &'a Person<'a>,
}

fn main() {
    let frank = Person {
        name: "Frank",
        age: 34,
    };

    let emp1 = Employee {
        id: "001",
        person: &frank,
    };

    println!("Employee id: {:?}", emp1.id);
    println!("Employee name: {:?}", emp1.person.name);
    println!("Employee age: {:?}", emp1.person.age);
}
