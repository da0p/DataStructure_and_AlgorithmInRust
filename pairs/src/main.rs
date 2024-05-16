use std::collections::HashMap;

/// Given a vector of numbers, returns all pairs that sum to a pre-defined sum
/// 
/// # Arguments
/// 
/// * `target_sum` - A pre-defined sum
/// 
/// * `numbers` - A vector of integers
/// 
/// # Notes
/// 
/// Complexity O(N)
/// 
fn pairs(target_sum: i32, numbers: &Vec<i32>) -> HashMap::<i32, i32> {
    let mut pairs = HashMap::<i32, i32>::new();
    let mut all_numbers = HashMap::<i32, i32>::new();

    for &first in numbers {
        let second = target_sum - first;
        all_numbers.insert(first, second);
    }

    for &first in numbers {
        let second = target_sum - first;
        if all_numbers.contains_key(&first) && all_numbers.contains_key(&second) && !pairs.contains_key(&second){
            pairs.insert(first, second);
        }
    }
    return pairs;
}

fn main() {
    let numbers = vec![0, 1, 4, 8, -3, 9, 7, 10];

    println!("{:#?}", pairs(7, &numbers));
}
