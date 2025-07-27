pub fn arrange_phrase(phrase: &str) -> String {
    let words: Vec<&str> = phrase.split_whitespace().collect();
    let word_count = words.len();
    let mut ordered_words: Vec<&str> = vec![""; word_count];

    for word in words {
        if let Some(digit_char) = word.chars().find(|c| c.is_digit(10)) {
            
            if let Some(position) = digit_char.to_digit(10) {
    
                let index = (position - 1) as usize;
                if index < word_count {
                    ordered_words[index] = word;
                }
            }
        }
    }
    
    let mut result = String::new();
    for (i, word) in ordered_words.iter().enumerate() {
      
        for char in word.chars().filter(|c| !c.is_digit(10)) {
            result.push(char);
        }

        if i < word_count - 1 {
            result.push(' ');
        }
    }
    result
}