use std::collections::HashMap;

///`smallest_distinct_window` finds the largest window contains all distinct
/// characters from a string
/// 
fn smallest_distinct_window(words: &str) -> String {
    let mut start = 0;
    let mut window_size = 0;
    let mut start_index = 0;
    let mut min_window_size = 0;
    let mut sub_window_dict = HashMap::new(); 
    while start + window_size < words.len() {
        let ch = words.chars().nth(start + window_size).unwrap();
        let cur_char = sub_window_dict.get(&ch);
        if cur_char == None {
            window_size += 1;
            sub_window_dict.insert(ch, 1);
        } else {
            if window_size > min_window_size {
                min_window_size = window_size;
                start_index = start;
            }
            start += 1;
            window_size = 0;
            sub_window_dict.clear();
        }
    }

    if window_size > min_window_size {
        min_window_size = window_size;
        start_index = start;
    }

    words[start_index..start_index + min_window_size].to_string()
}

fn main() {
    println!("smallest distinct window: {}", smallest_distinct_window("aabcbcdbcaad"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        assert_eq!(smallest_distinct_window("aabcbcdbcaaad"), "dbca");
    }

    #[test]
    fn all_different() {
        assert_eq!(smallest_distinct_window("abcdefgh"), "abcdefgh");
    }

    #[test]
    fn all_similar() {
        assert_eq!(smallest_distinct_window("aaaaaaaaa"), "a");
    }
}