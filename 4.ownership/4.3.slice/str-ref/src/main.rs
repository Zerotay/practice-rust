fn main() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_literal = "hello world";

    let word = first_word(&my_literal[0..6]);
    let word = first_word(&my_literal[..]);
    let word = first_word(my_literal);


    println!("{word}");
    println!("{my_literal}");
}

fn first_word(s:&str)->&str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}
