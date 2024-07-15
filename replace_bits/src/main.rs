fn replace_bits(n: i32, m: i32, i: u32, j: u32) -> i32 {
    if check_range(i, j) {
        return n;
    }

    extract_bits_in_range(m, i, j) | clear_bits_in_range(n, i, j)
}

fn clear_bits_in_range(n: i32, i: u32, j: u32) -> i32 {
    if check_range(i, j) {
        return n;
    }

    let mask = !((u32::MAX << i) & (u32::MAX >> (std::mem::size_of::<u32>() as u32 * 8 - j)));

    n & mask as i32
}

fn extract_bits_in_range(n: i32, i: u32, j: u32) -> i32 {
    if check_range(i, j) {
        return n;
    }

    let mask = (u32::MAX << i) & (u32::MAX >> (std::mem::size_of::<u32>() as u32 * 8 - j));

    n & mask as i32
}

fn check_range(i: u32, j: u32) -> bool
{
    if i > j {
        return false;
    }

    if i > std::mem::size_of::<i32>() as u32 - 1 || j > std::mem::size_of::<i32>() as u32 - 1 {
        return false;
    }

    true
}

fn main() {
    println!("{:#b}", clear_bits_in_range(0b11011011, 2, 6));
    println!("{:#b}", clear_bits_in_range(0b11011111, 2, 6));
    println!("{:#b}", replace_bits(0b11111111, 0b10000001, 2, 6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_bits_valid_range() {
        assert_eq!(clear_bits_in_range(0b11011011, 2, 6), 0b11000011);
    }

    #[test]
    fn invalid_lower_bound() {
        assert_eq!(clear_bits_in_range(0b11111111, 6, 2), 0b11111111);
    }

    #[test]
    fn invalid_upper_bound() {
        assert_eq!(clear_bits_in_range(0b11111111, 2, 50), 0b11111111);
    }

    #[test]
    fn extract_bits_valid_range() {
        assert_eq!(extract_bits_in_range(0b11011011, 2, 6), 0b00011000);
    }

    #[test]
    fn replace_bits_valid_range() {
        assert_eq!(replace_bits(0b11111111, 0b10000001, 2, 6), 0b11000011);
    }
}
