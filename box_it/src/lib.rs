pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut result: Vec<Box<u32>> = vec![];
    for mut val in s.split_whitespace() {
        if val.contains("k") {
            val = &val[0..(val.len()-1)];
            result.push(Box::new((val.parse::<f32>().unwrap()* 1000.0 )as u32));
        }else {
            result.push(Box::new(val.parse::<f32>().unwrap() as u32));
        }
    }
    result
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut result = vec![];
    for val in a {
        result.push(*val);
    }
    result
}