use std::vec;

/// Given a vector of numbers, returns all combinations of three numbers
/// that sums to a pre-defined sum
/// 
/// # Arguments
/// 
/// * `target_sum` - A pre-defined sum
///
/// * `numbers` - A sorted vector of numbers in ascending order
/// 
/// # Notes
/// 
/// This function use two-pointer algorithm that has O(N * N)
/// complexity
/// 
fn triplets(target_sum: u32, numbers: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut triplets = vec![];

    for (curr_i, &f_num) in numbers.iter().enumerate() {
        let mut i = curr_i + 1;
        let mut j = numbers.len() - 1;
        while i < j {
            let curr_sum = f_num + numbers[i] + numbers[j];
            if curr_sum == target_sum {
                triplets.push(vec![f_num, numbers[i], numbers[j]]);
                break;
            } else if curr_sum < target_sum {
                i += 1;
            } else {
                j -= 1;
            }
        }
    }
    return triplets;
}

fn main() {
    let numbers = vec![0, 1, 3, 7, 9, 10, 13, 17, 21, 25, 27];

    println!("{:#?}", triplets(30, &numbers));
}
