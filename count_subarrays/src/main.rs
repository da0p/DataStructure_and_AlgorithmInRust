use std::collections::HashMap;

///`count_sub_arrays` count the number of sub-arrays that sum to a pre-defined `sum`
/// 
fn count_sub_arrays(numbers: &Vec<i32>, sum: i32) -> u32 {
    let mut comp_sum = HashMap::new();
    let mut current_sum = 0;
    let mut counter = 0;
    let mut i = 0;
    while i < numbers.len() {
        current_sum += numbers[i];

        if current_sum == sum {
            counter += 1;
        }

        let comp_sum_with_current = current_sum - sum;
        let f_comp_sum = comp_sum.get(&comp_sum_with_current);
        if f_comp_sum != None {
            counter += *f_comp_sum.unwrap();
        }

        comp_sum.entry(current_sum).and_modify(|e| *e += 1).or_insert(1);

        i += 1;
    }

    counter
}
fn main() {
    let numbers = vec![10, 2, -2, -20, 10];
    println!("{:#?}", count_sub_arrays(&numbers, -10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input()
    {
        let numbers = vec![10, 2, -2, -20, 10];
        assert_eq!(count_sub_arrays(&numbers, -10), 3);
    }

    #[test]
    fn no_sub_array() {
        let numbers = vec![10, 2, -2, -20, 10];
        assert_eq!(count_sub_arrays(&numbers, 1), 0);
    }
}
