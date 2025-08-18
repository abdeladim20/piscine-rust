pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.is_empty() {
        return vec![];
    }

    let n = arr.len();
    let mut result = vec![1; n];
    let mut prefix_product = 1;
    for i in 0..n {
        result[i] = prefix_product;
        prefix_product *= arr[i];
    }

    let mut suffix_product = 1;
    for i in (0..n).rev() {
        result[i] *= suffix_product;
        suffix_product *= arr[i];
    }

    result
}