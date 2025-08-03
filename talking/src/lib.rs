pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }
    if text.trim().ends_with("?") && text == text.to_uppercase() && text.chars().any(|c| c.is_alphabetic()) {
        return "Quiet, I am thinking!";
    }else if text == text.to_uppercase() && text.chars().any(|c| c.is_alphabetic()) {
        return "There is no need to yell, calm down!";
    }else if text.trim().ends_with("?") {
        return "Sure.";
    }
    "Interesting"
}