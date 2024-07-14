use std::cmp::Ordering;

fn concatenate(numbers: &mut Vec<i32>) -> String {
    numbers.sort_by(|a, b| {
        let mut first_digits = to_digit(*a);
        let mut second_digits = to_digit(*b);
        sort_numbers(&mut first_digits, &mut second_digits)
    });

    numbers
        .iter()
        .rev()
        .map(|n| n.to_string())
        .collect::<String>()
}

fn sort_numbers(first: &mut Vec<i32>, second: &mut Vec<i32>) -> Ordering {
    while first.len() != 0 && second.len() != 0 {
        let first_digit = first.pop().unwrap();
        let second_digit = second.pop().unwrap();
        if first_digit != second_digit {
            return first_digit.partial_cmp(&second_digit).unwrap();
        }
    }
    return second.len().partial_cmp(&first.len()).unwrap();
}

fn to_digit(number: i32) -> Vec<i32> {
    let mut digits = vec![];
    if number == 0 {
        digits.push(0);
        return digits;
    }

    let mut divident = number;
    while divident != 0 {
        digits.push(divident % 10);
        divident = divident / 10;
    }
    return digits;
}

fn main() {
    let mut numbers = vec![10, 11, 20, 30, 3];
    println!("Biggest Number String: {:#?}", concatenate(&mut numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_number_smaller_than_10() {
        let number = 9;
        assert_eq!(to_digit(number), vec![9]);
    }

    #[test]
    fn convert_number_equal_to_0() {
        let number = 0;
        assert_eq!(to_digit(number), vec![0]);
    }

    #[test]
    fn convert_two_digit_number() {
        let number = 69;
        assert_eq!(to_digit(number), vec![9, 6]);
    }

    #[test]
    fn convert_three_digit_number() {
        let number = 696;
        assert_eq!(to_digit(number), vec![6, 9, 6]);
    }

    #[test]
    fn sort_unequal_length_number() {
        let mut number1 = to_digit(30);
        let mut number2 = to_digit(3);
        assert_eq!(sort_numbers(&mut number1, &mut number2), Ordering::Less);
    }

    #[test]
    fn sort_equal_length_number() {
        let mut number1 = to_digit(30);
        let mut number2 = to_digit(20);
        assert_eq!(sort_numbers(&mut number1, &mut number2), Ordering::Greater);
    }

    #[test]
    fn concatenate_sample() {
        let mut numbers = vec![10, 11, 20, 30, 3];
        let expected = String::from("330201110");
        assert_eq!(concatenate(&mut numbers), expected);
    }
}
