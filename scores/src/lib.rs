pub fn score(text: &str) -> u64 {
    let mut total: u64 = 0;
    for c in text.chars() {
        // A, E, I, O, U, L, N, R, S, T	
        let score = match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8, 
            'q' | 'z' => 10,
            _ => 0,
        };
        total += score as u64;
    }
    total
}