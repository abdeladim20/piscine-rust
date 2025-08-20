use generics_list::*;

fn main() {
    let mut new_list_str = List::new();
    new_list_str.push("String Test 1");
    // println!("The size of the list is {:?}", new_list_str);

    new_list_str.push("String Test 2");
    
    new_list_str.push("String Test 3");
    println!("The size of the list is {:#?}", new_list_str);
    // println!("The size of the list is ");

    println!("len: {:?}", new_list_str.len());

    new_list_str.pop();
    println!("The size of the list is {:?}", new_list_str);
}

/*
The size of the list is 1
The size of the list is 2
The size of the list is 3
The size of the list is 2
*/