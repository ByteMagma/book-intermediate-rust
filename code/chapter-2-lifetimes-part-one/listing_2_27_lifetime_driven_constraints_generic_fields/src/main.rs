struct CacheEntry<'a, T: 'a> {
    key: &'a str,
    value: T,
}

fn main() {
    let key = "session_token";
    let value = String::from("abc123");

    let entry = CacheEntry {
        key,
        value,
    };

    println!("Cache: {} => {}", entry.key, entry.value);
}
