use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut map = HashSet::new();
    for c in s.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            map.insert(c);
        }
    }
    if map.len() == 26 {
        return true;
    }
    false
}