fn longer<'a>(arg1: &'a str, arg2: &'a str) -> &'a str {
    if arg1.len() > arg2.len() {
        arg1
    } else {
        arg2
    }
}

fn main() {
    let longer_string;
    let msg1 = String::from("Message Received!");

    {
        let msg2 = String::from("Your message was received!");
        longer_string = longer(&msg1, &msg2);
        println!("Longer message: {}", longer_string);
    }
}
