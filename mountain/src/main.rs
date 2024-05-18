use std::vec;

/// 'mountain' is a function to find the longest band that strictly
/// ascends to a peak, then strictly descends.
/// 
/// Examples
/// 
///               5                4        4
///          3         2       1        3        2
///      1                 0                         1
/// -1 
/// 
/// The longest band is 6
/// 
/// ```
/// let numbers = vec![-1, 1, 3, 5, 2, 0, 1, 4, 3, 4, 2, 1];
/// assert_eq!(6, mountain(&numbers));
/// ```                                       
/// 
fn mountain(numbers: &Vec<i32>) -> u32 {
    let mut peaks = vec![];
    let mut i = 1;

    while i < numbers.len() - 1 {
        if numbers[i] > numbers[i - 1] && numbers[i] > numbers[i + 1] {
            peaks.push(i);
        }
        i += 1;
    }

    let mut highest_mount = 0;
    for peak in peaks {
        i = peak;
        let mut left = 0;
        while numbers[i] > numbers[i - 1] {
            left += 1;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        i = peak;
        let mut right = 0;
        while i + 1 < numbers.len() && numbers[i] > numbers[i + 1] {
            right += 1;
            i += 1;
        }

        if left + right > highest_mount {
            highest_mount = left + right;
        }
    }

    return highest_mount;
}

fn main() {
    let numbers = vec![1, -1, 5, 6, 7, 9, 10, 6, 5, 2, 7, 9, 40, 50, 20, 10, 0, -20, -50, -100, 10, 3, 2];

    println!("Highest mountain: {}", mountain(&numbers));
}
