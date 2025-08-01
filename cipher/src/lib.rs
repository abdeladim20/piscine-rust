#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut result = String::new();
    for c in original.chars() {
        if c.is_lowercase() && c.is_alphabetic() {
            result.push(('z' as u8 - (c as u8 - 'a' as u8)) as char);
        } else if c.is_uppercase() && c.is_alphabetic() {
            result.push(('Z' as u8 - (c as u8  - 'A' as u8)) as char);
        } else {
            result.push(c);
        }
    }
    if result == ciphered {
        return Ok(());
    }
    Err(CipherError { expected: result })
}