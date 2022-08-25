use std::collections::HashMap;
use std::collections::HashSet;

pub fn char_freq(word: &str) -> HashMap<char, u32> {
    let mut to_ret: HashMap<char, u32> = HashMap::new();
    for c in word.chars() {
        *(to_ret.entry(c).or_insert(0)) += 1;
    }
    to_ret
}

pub fn is_anagram(query: String, mut clone_map: HashMap<char, u32>) -> bool {
    for c in query.chars() {
        // if (!clone_map.contains_key(&c)){
        //     return false;
        // }
        let val = clone_map.get_mut(&c);
        match val {
            None => {
                return false;
            }
            Some(x) => {
                if *x == 0 {
                    return false;
                }
                *x -= 1;
            }
        }
    }

    for (_, value) in clone_map {
        if value > 0 {
            return false;
        }
    }
    true
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    println!("ref word: {word}");
    let mut to_ret: HashSet<&'a str> = HashSet::new();
    let low_word = word.to_lowercase();
    println!("ref low_word: {low_word}");
    let word_char_freq = char_freq(low_word.as_str());
    let word_length = word.chars().count();
    for text in possible_anagrams {
        println!("text {text}");
        if text.chars().count() != word_length {
            continue;
        }
        let low_text = text.to_lowercase();
        if low_text == low_word {
            continue;
        }
        let clone_map = word_char_freq.clone();
        if is_anagram(low_text, clone_map) {
            to_ret.insert(text);
        }
        // let text_freq = char_freq(low_text);
    }
    // to_ret.take(&word);
    // to_ret.iter().cloned().collect()
    to_ret.iter().map(|&x| x.clone()).collect()
    // unimplemented!(
    //     "For the '{}' word find anagrams among the following words: {:?}",
    //     word,
    //     possible_anagrams
    // );
}
