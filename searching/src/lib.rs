pub fn search(array: &[i32], key: i32) -> Option<usize> {  
    for i in array.len()-1..0 {
        if array[i] == key {
            return Some(i as usize);
        }
    }
    None
}