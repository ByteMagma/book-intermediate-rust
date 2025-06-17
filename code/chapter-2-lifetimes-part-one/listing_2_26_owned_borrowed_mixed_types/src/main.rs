struct Document<'a> {
    id: u32,
    title: String,
    body: &'a str,
}

fn main() {
    let body_text = String::from("Rust lifetimes are powerful and safe.");
    
    let doc = Document {
        id: 42,
        title: "Learning Lifetimes".to_string(),
        body: &body_text,
    };

    println!("Doc #{}: {}", doc.id, doc.body);
}
