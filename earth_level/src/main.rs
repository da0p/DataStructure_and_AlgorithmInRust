fn earth_level(level: u32) -> u32
{
    if level == 0 {
        return 0;
    }

    let mut steps = 1;
    let mut new_level = level - sig_num(level);
    while new_level > 0 {
        new_level = new_level - sig_num(new_level);
        steps += 1;
    }
    steps
}

fn sig_num(num: u32) -> u32 {
    let mut x = num;
    let mut count = 0;
    x = x >> 1;
    while x > 0 {
        x = x >> 1;
        count += 1;
    }

    1 << count
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closest_power_of_two()
    {
        assert_eq!(sig_num(0b1101), 0b1000);
    }

    #[test]
    fn find_steps_1() {
        assert_eq!(earth_level(7), 3);
    }

    #[test]
    fn find_steps_2() {
        assert_eq!(earth_level(8), 1);
    }

    #[test]
    fn find_steps_3() {
        assert_eq!(earth_level(0), 0);
    }

    #[test]
    fn find_steps_4() {
        assert_eq!(earth_level(1), 1);
    }
}