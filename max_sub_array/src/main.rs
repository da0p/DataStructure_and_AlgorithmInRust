use core::cmp;
use std::vec;

/// `max_sub_array` finds the max subarray sum that can be formed.
/// A sub array is defined as consecutive segment of the array.
/// If all numbers are negative, then return 0.
/// 
/// Examples
/// 
/// {-1, 2, 3, 4, -2, 6, -8, 3}
/// 
/// Output = 13
/// 
fn max_sub_array(numbers: &Vec<i32>) -> i32 {
    let mut max = 0;
    let mut sum = 0;
    let mut i = 0;

    while i < numbers.len() {
        if numbers[i] > 0 {
            break;
        }
        i += 1;
    }

    while i < numbers.len() {
        if numbers[i] < 0 && (i == numbers.len() - 1 || numbers[i] + numbers[i + 1] < 0) {
            max = cmp::max(max, sum);
            sum = 0;
        } else {
            sum += numbers[i];
        }
        i += 1;
    }

    max = cmp::max(max, sum);

    return max;
}

fn main() {
    let numbers = vec![-1, 2, 3, 4, -2, 6, -8, 3];
    println!("max = {}", max_sub_array(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monotonic_increase() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(max_sub_array(&numbers), 21);
    }

    #[test]
    fn only_negative() {
        let numbers = vec![-1, -2, -3, -4];
        assert_eq!(max_sub_array(&numbers), 0);
    }

    #[test]
    fn negative_at_tail() {
        let numbers = vec![1, 2, 3, 4, -5];
        assert_eq!(max_sub_array(&numbers), 10);
    }

    #[test]
    fn negative_at_front() {
        let numbers = vec![-5, 1, 2, 3, 4];
        assert_eq!(max_sub_array(&numbers), 10);
    }

    #[test]
    fn negative_at_middle() {
        let numbers = vec![1, 2, 3, -4, 1, 2, 4];
        assert_eq!(max_sub_array(&numbers), 7);
    }

    #[test]
    fn two_negative() {
        let numbers = vec![1, 2, 3, -4, 1, 2, 5, -6, 1, 2, 1];
        assert_eq!(max_sub_array(&numbers), 8);
    }
}
