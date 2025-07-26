pub fn nbr_function(c: i32) -> (i32, f64, f64) { 
    let f = c as f64;
    let l = f.abs().ln();
    (c, f.exp(), l)
}

pub fn str_function(a: String) -> (String, String) {
    let mut splited = String::new();
    for val in a.split_whitespace() {
        let convert :f64;
        convert = val.parse::<f64>().expect("Failed to parse string to float64");
        let c = convert.exp();
        splited.push_str(&(c.to_string()+" "));
    }
    (a, splited.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut result = Vec::new();
    for num in &b {
        let c = *num as f64;
        let ln = c.ln().abs();
        result.push(ln);
    }
    (b, result)
}