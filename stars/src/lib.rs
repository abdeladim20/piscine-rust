pub fn stars(n: u32) -> String {
    let stars = 2_i32.pow(n);
    let mut result = String::new();
    for _ in 0..stars {
        result.push('*');
    }
    result
}