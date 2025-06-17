struct Message<'a> {
    content: &'a str,
}

impl<'a> Message<'a> {
    fn get_content(&self) -> &'a str {
        self.content
    }
}

fn main() {
    let msg = String::from("Welcome back!");
    let message = Message { content: &msg };
    println!("Message content: {}", message.get_content());
}
