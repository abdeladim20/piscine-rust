pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();
    for words in names {
        let mut w = String::new();
        for word in words.split_whitespace() {
            w.push_str(&(word.chars().nth(0).expect("").to_string()+". "));
        }
        result.push(w.trim().to_string());
    }
    result
}