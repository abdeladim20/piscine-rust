pub fn first_fifty_even_square() -> Vec<i32> {
    let mut result = Vec::new();
    let square = |n: i32| n * n;
    (1..=100).for_each(|n: i32| {
        if n % 2 == 0 {
            result.push(square(n));
        }
    });
    result
}