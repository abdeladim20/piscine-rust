pub fn scytale_cipher(message: &str, i: usize) -> String {
    if i <= 1 || message.is_empty() {
        return if i == 0 { String::new() } else { message.to_string() };
    }

    let mut chars: Vec<char> = message.chars().collect();
    let len = chars.len();

    let rows = (len + i - 1) / i;
    let grid_size = rows * i;
    chars.resize(grid_size, ' ');

    let mut result = String::with_capacity(grid_size);

    
    for col in 0..i {
        for current_index in (col..grid_size).step_by(i) {
            result.push(chars[current_index]);
        }
    }    
    result.trim_end().to_string()
}