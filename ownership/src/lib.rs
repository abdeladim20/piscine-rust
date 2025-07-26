pub fn first_subword(mut s: String) -> String {
    if s.contains("_") {
        return s.split("_").nth(0).expect("Error: didn't find '_'").to_owned();
    }
    for (i, v) in s.chars().enumerate() {
        if v >= 'A' && v<= 'Z' && i > 0 {
            return s[0..i].to_owned();
        }
    }
    s
}