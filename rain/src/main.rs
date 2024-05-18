use std::vec;

/// `rain` is a function to find the total amount of rain trapped
/// 
/// Examples                          
///                                    ----
///                                   |    |
///                ----               .     ----      ----
///               |    |              |         |    |    |
///      ----     .    .----      ----           ----      ----
///     |    |    |         |    |                             |
/// ----      ----           ---- 
///   0    1    0    2    1    0    1    3    2    1    2    1
/// 
/// The total amount of rain trapped is 6
/// 
/// Complexity O(n)
/// 
fn rain(numbers: &Vec<u32>) -> u32 {

    assert!(numbers.len() > 0);

    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    let mut max = 0;
    for &number in numbers {
        if number > max {
            max = number;
        }
        left.push(max);
    }

    max = 0;
    for &number in numbers.iter().rev() {
        if number > max {
            max = number;
        }
        right.push(max);
    }
    right.reverse();

    let mut i = 0;
    let mut rain = vec![];
    while i < numbers.len() {
        let quantity = std::cmp::min(left[i], right[i]) - numbers[i];
        rain.push(quantity);
        i += 1;
    }

    return rain.iter().sum();
}

fn main() {
    let numbers = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("Total rain: {}", rain(&numbers));
}
