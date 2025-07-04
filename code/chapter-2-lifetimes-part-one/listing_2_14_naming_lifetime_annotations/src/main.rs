struct Person<'pers> {
    name: &'pers str,
    age: u8,
}
struct Employee<'id, 'p> {
    id: &'id str,
    person: &'p Person<'p>,
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
