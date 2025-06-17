struct Message<'a> {
    content: &'a str,
}

impl<'a> Message<'a> {
    fn is_prefix(&self, prefix: &str) -> bool {
        self.content.starts_with(prefix)
    }
}

fn main() {
    let msg = String::from("Processing payment...");
    let message = Message { content: &msg };

    if message.is_prefix("Processing") {
        println!("Message is processing something.");
    } else {
        println!("Different message.");
    }
}
