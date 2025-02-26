fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();
        
    for (i, &item) in bytes.iter().enumerate().rev()  {
        if item == b' ' {
            return &s[i+1..];
        }
    }

    &s[..]
}


fn main() {
    let s = String::from("hello world, great to see you!");
    let word = first_word(&s);
    println!("The first word is: {word}"); 

    let word2 = first_word(&s[6..]);
    println!("after 6+ chr,  the first word is: {word2}"); 

    let word3 = last_word(&s[..]);
    println!("the last word is: {word3}");
}
