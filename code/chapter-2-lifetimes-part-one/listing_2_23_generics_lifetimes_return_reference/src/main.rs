fn longer_string<'a, T: AsRef<str>>(str_one: &'a T, str_two: &'a T) -> &'a str {
    let one = str_one.as_ref();
    let two = str_two.as_ref();
    if one.len() > two.len() {
        one
    } else {
        two
    }
}
fn main() {
    let s1 = String::from("Database");
    let s2 = String::from("Cache");

    let result = longer_string(&s1, &s2);
    println!("Longer label: {}", result);
}
