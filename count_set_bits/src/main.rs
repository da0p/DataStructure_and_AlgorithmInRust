fn count_set_bits(n: u32) -> u32 {
    let mut num = n;
    let mut counter: u32 = 0;
    while num > 0 {
        num = num & (num - 1);
        counter += 1;
    }
    counter
}

fn main() {
    println!("Number of set bits: {}", count_set_bits(9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_set_bits_of_nine() {
        assert_eq!(count_set_bits(9), 2);
    }

    #[test]
    fn count_set_bits_of_eight() {
        assert_eq!(count_set_bits(8), 1);
    }
}