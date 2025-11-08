fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);

    // -----------------------------------------
    let s_literal = "hello world";

    let word_literal = first_word(s_literal);

    println!("the first word is: {}", word_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}
