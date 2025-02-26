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

fn third_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    let mut count = 0;
    let mut i1 = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            count += 1;
            if count == 2 {
                i1 = i;
            } else if count == 3 {
                return &s[i1+1..i];
            }
        }
    }

    // return empty string if there are less than 3 words
    ""
}


fn main() {
    let s = String::from("hello world, great to see you!");
    let word = first_word(&s);
    println!("The first word is: {word}"); 

    let word2 = first_word(&s[6..]);
    println!("after 6+ chr,  the first word is: {word2}"); 

    let word3 = third_word(&s[..]);
    println!("the third word is: {word3}");

    let word4 = last_word(&s[..]);
    println!("the last word is: {word4}");
}
