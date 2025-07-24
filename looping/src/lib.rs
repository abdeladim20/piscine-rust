use std::io; 

pub fn check() -> u16 {
    let mut counter = 0;
    loop {
        let mut input = String::new();
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        counter+=1;
        let _ = io::stdin().read_line(&mut input);
        let result = input.trim();
        if  result == "The letter e"{
                return counter;
            }
    }
}