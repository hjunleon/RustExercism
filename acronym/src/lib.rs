fn is_whitespace(c: char)-> bool {
    if c == ' ' || c == '-' || c == '_' || c ==',' {
        return true
    }
    false
}

fn is_ignore(c: char) -> bool{
    c == '\''
}

fn is_letter_upcase(c: char)->bool {
    !c.is_ascii_lowercase()
}

pub fn abbreviate(phrase: &str) -> String {
    let mut char_vec:  Vec<char> = Vec::new();
    // let mut seen_word: bool = false;
    let mut is_prev_lower: bool = true;
    let mut is_prev_space: bool = true;
    for idx in 0..phrase.len(){
        let cur_char = phrase.chars().nth(idx).unwrap();
        if is_ignore(cur_char){
            continue;
        }
        if !is_whitespace(cur_char) {
            if is_letter_upcase(cur_char){
                if is_prev_space || is_prev_lower{
                    char_vec.push(cur_char.to_ascii_uppercase());
                }
                is_prev_lower = false;
            } else {
                if is_prev_space {
                    char_vec.push(cur_char.to_ascii_uppercase());
                }
                is_prev_lower = true;
            }
            is_prev_space = false;
        } else {
            is_prev_space = true;
        }
    }
    char_vec.into_iter().collect()
    // String::from_utf8_lossy()
    // unimplemented!("Given the phrase '{}', return its acronym", phrase);
}
