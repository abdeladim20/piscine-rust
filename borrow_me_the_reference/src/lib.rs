pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut counter = 0;
    for val in s.chars() {
        if val == '-' {
            result.pop();
        } else if  val == '+' {
            counter+=1; 
        }else {
            if counter > 0 {
                counter-=1;
                continue;
            }
            result.push(val);
        }
    }
    *s = result;
}

pub fn do_operations(v: &mut [String]) {
// first method!
//   if val.contains("+") {
//             let res = val.as_str().split("+").collect::<Vec<&str>>();
//             let result = res.iter().nth(0).expect("").parse::<i32>().unwrap() + res.iter().nth(1).expect("").parse::<i32>().unwrap();
//             *val = result.to_string();
//         }else {
//             let res = val.as_str().split("-").collect::<Vec<&str>>();
//             let result = res.iter().nth(0).expect("").parse::<i32>().unwrap() - res.iter().nth(1).expect("").parse::<i32>().unwrap();
//             *val = result.to_string();
//         }
    for val in v {
        *val = match val.find("+") {
            Some(i) => (&val[0..i].parse::<i32>().unwrap()+ &val[i+1..].parse::<i32>().unwrap()).to_string(), 
            None =>{
              match val.find("-") {
                Some(i) => (&val[0..i].parse::<i32>().unwrap() - &val[i+1..].parse::<i32>().unwrap()).to_string(),
                None =>  "".to_string(),
                }  
            },
        }
    }
}