pub fn number_logic(num: u32) -> bool {
    let num_in_str = num.to_string();
    let number_count = num_in_str.len() as u32;
    let mut result: u64 = 0; 
    for c in num_in_str.chars() {
        result += c.to_digit(10).unwrap().pow(number_count) as u64;
    }
    result == num as u64
}