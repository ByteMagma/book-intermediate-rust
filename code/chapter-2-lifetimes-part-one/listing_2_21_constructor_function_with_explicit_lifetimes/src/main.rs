struct Record<'title, 'body> {
    title: &'title str,
    body: &'body str,
}

impl<'title, 'body> Record<'title, 'body> {
    fn new(title: &'title str, body: &'body str) -> Self {
        Record { title, body }
    }

    fn summary(&self) -> &str {
        if self.title.len() > self.body.len() {
            self.title
        } else {
            self.body
        }
    }
}

fn main() {
    let config_title = String::from("System Alert");
    let user_input = String::from("Low disk space on volume /dev/sda1.");

    let record = Record::new(&config_title, &user_input);
    
    let summary_result = record.summary(); // safe: both live long enough
    println!("Summary: {}", summary_result);
}
