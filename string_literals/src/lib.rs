pub fn is_empty(v: &str) -> bool {
    return v.len() == 0;
}

pub fn is_ascii(v: &str) -> bool {
    return v.is_ascii();
}

pub fn contains(v: &str, pat: &str) -> bool {
    return v.contains(pat);
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    let (first_part, second_part) = v.split_at(index);
    return (first_part, second_part);
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).expect("Error: match does not found")
}