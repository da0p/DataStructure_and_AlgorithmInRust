use std::vec;

/// 'sub_array_1' and `sub_array_2` are functions to find the sub-array that
/// is out-of-order
/// 
/// Examples
/// 
/// 1   2   3   4   5   8   6   7   9   10   11
///                     |       | 
///                     5       7 
/// A pair (5, 7) will be returned indicating the starting index and ending
/// index of the out-of-order sub-array
/// 
/// Notes
/// 
/// `sub_array_1` has O(NlogN) complexity
/// `sub_array_2` has O(N) complexity
/// 
fn sub_array_1(numbers: &Vec<i32>) -> (usize, usize)
{
    let mut sub_array = (0, 0);

    let mut sorted_numbers = numbers.clone();

    sorted_numbers.sort();

    for i in 0..numbers.len() {
        if numbers[i] != sorted_numbers[i] {
            sub_array.0 = i;
            break;
        }
    }

    for i in (0..numbers.len()).rev() {
        if numbers[i] != sorted_numbers[i] {
            sub_array.1 = i;
            break;
        }
    }

    return sub_array;
}

fn sub_array_2(numbers: &Vec<i32>) -> (usize, usize)
{
    let mut sub_array = (0, 0);

    let mut max = std::i32::MIN;
    let mut min = std::i32::MAX;

    for i in 1..numbers.len() - 1 {
        if numbers[i] > numbers[i - 1] && numbers[i] < numbers[i + 1] {
            continue;
        }

        if numbers[i] > max {
            max = numbers[i];
        }

        if numbers[i] < min {
            min = numbers[i];
        }
    }

    for i in 0..numbers.len(){
        if min < numbers[i] {
            sub_array.0 = i;
            break;
        } 
    }

    for i in (0..numbers.len()).rev() {
        if max > numbers[i] {
            sub_array.1 = i;
            break;
        }
    }

    return sub_array;
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 8, 6, 7, 9, 10, 11];
    println!("{:#?}", sub_array_1(&numbers));
    println!("{:#?}", sub_array_2(&numbers));
}
