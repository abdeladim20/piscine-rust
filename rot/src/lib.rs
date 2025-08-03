pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();
    for c in input.chars() {
        let mut new_char = c;
        if c.is_alphabetic() {
            if c.is_lowercase() {
                let base = 'a';
                let position = c as u8 - base as u8;
                let rotated_position = (position as i16 + key as i16).rem_euclid(26);
                new_char = (base as u8 + rotated_position as u8) as char;
            }else if c.is_uppercase() {
                let base = 'A';
                let position = c as u8 - base as u8;
                let rotated_position = (position as i16 + key as i16).rem_euclid(26);
                new_char = (base as u8 + rotated_position as u8) as char;
            }
        }
        result.push(new_char);
        
    }
    result
}