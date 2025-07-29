pub fn capitalize_first(input: &str) -> String {
    let mut result = input.chars().collect::<Vec<char>>();
    result[0].make_ascii_uppercase();
    result.iter().collect::<String>()
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new(); 
    for val in input.split_whitespace() {
        result.push_str(&(capitalize_first(val) + " "));
    }

    result.trim().to_string()
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_lowercase() {
            result.push(c.to_ascii_uppercase());
        }else if c.is_uppercase() {
            result.push(c.to_ascii_lowercase());
        }else {
            result.push(c);
        };
    }
    result
}