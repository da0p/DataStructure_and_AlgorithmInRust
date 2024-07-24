///`housing` finds all ranges that has the sum which is equal to a defined `sum`
/// 
/// Note: Complexity O(N)
/// 
fn housing(numbers: &Vec<u32>, sum: u32) -> Vec<(usize, usize)>
{
    let mut begin = 0;
    let mut end = 0;
    let mut cur_sum = 0;
    let mut indices = vec![];

    while end < numbers.len() {
        cur_sum += numbers[end];
        end += 1;

        while cur_sum > sum && begin < end {
            cur_sum -= numbers[begin];
            begin += 1;
        }

        if cur_sum == sum {
            indices.push((begin, end - 1));
        }
    }

    indices
}

fn main() {
    let numbers = vec![1, 3, 2, 1, 4, 1, 3, 2, 1, 1];
    println!("{:#?}", housing(&numbers, 8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_array() {
        let numbers = vec![1, 3, 2, 1, 4, 1, 3, 2, 1, 1];
        assert_eq!(housing(&numbers, 8), vec![(2, 5), (4, 6), (5, 9)]);
    }

    #[test]
    fn array_contains_number_equal_to_sum() {
        let numbers = vec![1, 8, 2, 1, 4, 1, 3, 2, 1, 1];
        assert_eq!(housing(&numbers, 8), vec![(1, 1), (2, 5), (4, 6), (5, 9)]);
    }

    #[test]
    fn array_contains_number_larger_than_sum() {
        let numbers = vec![1, 11, 2, 1, 4, 1, 3, 2, 1, 1];
        assert_eq!(housing(&numbers, 8), vec![(2, 5), (4, 6), (5, 9)]);
    }
}
