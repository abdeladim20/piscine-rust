pub fn is_vowel(c: char) -> bool {
    "aeuio".contains(c.to_ascii_lowercase())
}

pub fn pig_latin(text: &str) -> String {
    let mut result = String::new();
    let c = text.chars().next().unwrap();
    if is_vowel(c) {
        result.push_str(text);
        result.push_str("ay");
        return result;
    }

    if text.len() >= 3 {
        if &text[1..=2] == "qu" {
            result.push_str(&text[3..]);
            result.push_str(&text[0..=2]);
            result.push_str("ay");
            return result;
        }
    }
    let mut i = 0;
    for c in text.chars() {
        i += 1;
        if is_vowel(c) {
            result.push_str(&text[i-1..]);
            result.push_str(&text[..i]);
            result.push_str("ay");
            return result;
        }
    }
    result
}

