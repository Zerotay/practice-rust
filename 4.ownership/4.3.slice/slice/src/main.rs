fn main() {
    let mut s = String::from("hello world");
    let fw = first_word(&s);
    s.clear();
    //println!("first_word is {fw}");
    
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
