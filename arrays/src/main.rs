use arrays::*;

fn main() {
    let a :&[i32] = &(1..=10).collect::<Vec<i32>>();
    let b = [5; 10];

    println!("The sum of the elements in {:?} is {}", a, sum(&a));
    println!("The sum of the elements in {:?} is {}", b, sum(&b));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}

/* 
The sum of the elements in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] is 55
The sum of the elements in [5, 5, 5, 5, 5, 5, 5, 5, 5, 5] is 50
Array of 32 elements filled with 10 = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]
*/