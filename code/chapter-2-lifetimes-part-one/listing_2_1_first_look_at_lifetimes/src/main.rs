fn first_word<'a>(msg: &'a str) -> &'a str {
    match msg.find(' ') {
        Some(pos) => &msg[..pos],
        None => msg,
    }
}

fn main() {
    let message = "Frank is a premium user.";
    let name = first_word(&message);
    println!("Username: {}", name);
}
