use std::{collections::HashMap, vec};

/// Given a vector of numbers, returns the minimum swaps to sort the vector
///
/// # Arguments
/// 
/// * `numbers` - A vector of numbers
/// 
/// # Notes
/// 
/// The complexity of this algorithm is O(N * logN)
/// 
fn min_swap(numbers: &Vec<u32>) -> u32 {
    let num_to_index = numbers
        .iter()
        .enumerate()
        .map(|(index, num)| (*num, index))
        .collect::<HashMap<u32, usize>>();

    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    let mut visited = vec![false; sorted_numbers.len()];

    let mut swaps = 0;
    let mut i = 0;

    while i < sorted_numbers.len() {
        if !visited[i] {
            let mut j = i;
            visited[j] = true;
            j = *num_to_index.get(&sorted_numbers[j]).unwrap();
            while !visited[j] {
                j = *num_to_index.get(&sorted_numbers[j]).unwrap();
                swaps += 1;
            }
        }
        i += 1;
    }
    return swaps;
}

fn main() {
    let numbers = vec![2, 4, 5, 1, 3];
    println!("min swaps = {}", min_swap(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_swap_needed() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(min_swap(&numbers), 0);
    }

    #[test]
    fn two_swaps() {
        let numbers = vec![5, 4, 3, 2, 1];
        assert_eq!(min_swap(&numbers), 2);
    }

    #[test]
    fn three_swaps() {
        let numbers = vec![2, 4, 5, 1, 3];
        assert_eq!(min_swap(&numbers), 3);
    }
}
