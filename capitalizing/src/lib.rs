pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut result: Vec<char> = input.chars().collect();
    result[0].make_ascii_uppercase();
    result.into_iter().collect()
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize = true;
        } else if capitalize {
            result.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            result.push(c);
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_uppercase() {
            result.push(c.to_ascii_lowercase());
        } else if c.is_lowercase() {
            result.push(c.to_ascii_uppercase());
        } else {
            result.push(c);
        }
    }
    result
}