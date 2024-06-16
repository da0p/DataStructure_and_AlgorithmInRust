use std::vec;

/// `product_array` takes in a vector of integers, and returns
/// a vector of the same length, where each eleent in the output
/// array is equal to the product of every other number in the
/// input array. Solve this problem without using division
/// 
/// Note: Complexity O(N)
/// 
fn product_array(numbers: &Vec<u32>) -> Vec<u32>
{
    let mut prod = vec![1; numbers.len()];
    let mut left = vec![1; numbers.len()];
    let mut right = vec![1; numbers.len()];

    for i in 1..numbers.len() {
        left[i] = left[i - 1] * numbers[i - 1];
    }

    for i in (0..numbers.len() - 1).rev() {
        right[i] = right[i + 1] * numbers[i + 1];
    }

    for i in 0..numbers.len() {
        prod[i] = left[i] * right[i];
    }

    return prod;
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Product = {:#?}", product_array(&numbers));
}
