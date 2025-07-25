pub fn to_url(s: &str) -> String {
    let new_s = s.replace(" ", "%20");
    return new_s;
}