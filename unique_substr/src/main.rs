use std::collections::HashSet;

struct Index {
    begin: usize,
    end: usize,
}

fn unique_substr(ascii_str: &str) -> String {
    let mut ascii_chars = HashSet::new();
    let mut cur_ind = Index { begin: 0, end: 0 };
    let mut max_ind = Index { begin: 0, end: 0 };

    while cur_ind.end < ascii_str.len() {
        let cur_char = ascii_str.chars().nth(cur_ind.end).unwrap();
        if !ascii_chars.contains(&cur_char) {
            ascii_chars.insert(cur_char);
            cur_ind.end += 1;
        } else {
            if cur_ind.end - cur_ind.begin > max_ind.end - max_ind.begin {
                max_ind.begin = cur_ind.begin;
                max_ind.end = cur_ind.end;
            }
            ascii_chars.clear();
            cur_ind.begin = cur_ind.end;
        }
    }

    if max_ind.begin == max_ind.end && max_ind.begin == 0 {
        max_ind.end = ascii_str.len();
    }

    ascii_str[max_ind.begin..max_ind.end].to_string()
}

fn main() {
    println!("Substr = {}", unique_substr("thisiscool"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_string() {
        let ascii_str = "thisiscool";
        assert_eq!(unique_substr(ascii_str), "this");
    }

    #[test]
    fn no_duplicate_string() {
        let ascii_str = "abcdefgh";
        assert_eq!(unique_substr(ascii_str), "abcdefgh");
    }
}
