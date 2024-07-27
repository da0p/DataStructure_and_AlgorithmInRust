use std::collections::VecDeque;

///`sliding_window_max` finds the maximum number of consecutive subwindow
/// 
fn sliding_window_max(numbers: &Vec<usize>, size: usize) -> Vec<usize> {
    let mut window_max = vec![];
    let mut deque_max = VecDeque::new();
    let mut i = 0;

    if numbers.len() < size {
        return vec![];
    }

    while i < size {
        if !deque_max.is_empty() && numbers[i] > numbers[*deque_max.back().unwrap()] {
            deque_max.pop_back();
        }
        deque_max.push_back(i);
        i += 1;
    }

    while i < numbers.len() {
        let front_index = *deque_max.front().unwrap();
        window_max.push(numbers[front_index]);

        // remove elements that is outside of the current window
        while !deque_max.is_empty() && *deque_max.front().unwrap() <= i - size {
            deque_max.pop_front();
        }

        while !deque_max.is_empty() && numbers[*deque_max.back().unwrap()] <= numbers[i] {
            deque_max.pop_back();
        }

        deque_max.push_back(i);
        
        i += 1;
    }

    let front_index = *deque_max.front().unwrap();
    window_max.push(numbers[front_index]);

    window_max
}

fn main() {
    let numbers = vec![1, 2, 3, 1, 4, 5, 2, 3, 6];
    println!("window_max = {:#?}", sliding_window_max(&numbers, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let numbers = vec![1, 2, 3, 1, 4, 5, 2, 3, 6];
        assert_eq!(sliding_window_max(&numbers, 3), vec![3, 3, 4, 5, 5, 5, 6]);
    }

    #[test]
    fn size_larger_than_numbers_length() {
        let numbers = vec![1, 2, 3, 4];
        assert_eq!(sliding_window_max(&numbers, 5), vec![]);
    }

    #[test]
    fn size_equal_numbers_length() {
        let numbers = vec![1, 2, 3, 4];
        assert_eq!(sliding_window_max(&numbers, 4), vec![4]);
    }
}