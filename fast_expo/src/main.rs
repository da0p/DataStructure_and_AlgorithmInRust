fn fast_exp(x: u32, n: u32) -> u128 {
    let mut exp: u128 = 1;
    let mut p = n;
    let mut q = x;
    while p > 0 {
        let last_bit = p & 1;
        if last_bit == 1 {
            exp = exp * q as u128;
        }
        q = q * q;
        p = p >> 1;
    }

    exp
}

fn main() {
    println!("2^10 = {}", fast_exp(2, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_power_ten() {
        assert_eq!(fast_exp(2, 10), 1024);
    }

    #[test]
    fn three_power_five() {
        assert_eq!(fast_exp(3, 5), 243);
    }
}
