struct Message<'a> {
    content: &'a str,
}

impl<'a> Message<'a> {
    fn longest<'b>(&'a self, other: &'b str) -> &'a str {
        if self.content.len() >= other.len() {
            self.content
        } else {
            // In a real-world case, you'd return `other` only if it lived as long as `'a`
            // Here we just return `self.content` for illustration
            self.content
        }
    }
}

fn main() {
    let saved = String::from("Hello, user.");
    let message = Message { content: &saved };

    let temp = String::from("Hi");

    let result = message.longest(&temp);
    println!("Longest: {}", result);
}
