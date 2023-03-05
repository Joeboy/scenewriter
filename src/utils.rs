pub fn truncate_string(ss: &String, num_chars: Option<usize>) -> String {
    let mut s = ss.clone();
    let num_chars = num_chars.unwrap_or(20);
    if s.chars().count() > num_chars {
        s.truncate(s.chars().take(num_chars).map(|c| c.len_utf8()).sum());
        s.push_str("...");
    }
    s
}

pub fn print_underlined(s: &str) {
    println!("{}", s);
    println!("{}", "-".repeat(s.len()));
}