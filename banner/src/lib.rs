use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag{
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Flag {
            short_hand: format!("-{}", name.chars().nth(0).unwrap()),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let result = self.flags.get(input);
        match result {
            Some(func) => match func(argv[0], argv[1]) {
                Ok(value) => {return Ok(value);}
                Err(err) => {return Err(err.to_string());}
            },
            None => {return Err("flag not found".to_string());}
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num_a = a.parse::<f64>()?;
    let num_b = b.parse::<f64>()?;
    // num_a/num_b;
    Ok((num_a / num_b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num_a = a.parse::<f64>()?;
    let num_b = b.parse::<f64>()?;
    // num_a%num_b;
    Ok((num_a%num_b).to_string())
}