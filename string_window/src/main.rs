use std::collections::HashMap;
/// `string_window` finds the smallest window of string that contains 
/// all characters in the provided pattern
/// 
/// Example
/// 
/// words = "hello"
/// pattern = "elo"
/// assert_eq!(string_window(&words, &pattern), "ello")
/// 
/// Complexity O(n)
/// 
fn string_window(words: &str, pattern: &str) -> String {

    let mut counter = 0;
    let mut start = 0;
    let mut start_index = 0;
    let mut min_window_size = usize::MAX;
    let mut pat_freq = HashMap::new();
    let mut word_freq = HashMap::new();
    for p in pattern.chars() {
        pat_freq.entry(p).and_modify(|e| *e += 1).or_insert(1);
    }

    for (i, cur_char) in words.char_indices() {
        word_freq.entry(cur_char).and_modify(|e| *e += 1).or_insert(1);

        let lookup_word_freq = word_freq[&cur_char];
        let lookup_pat_freq = pat_freq.get(&cur_char);
        if lookup_pat_freq != None && lookup_word_freq <= *lookup_pat_freq.unwrap()
        {
            counter += 1;
        }
        if counter == pattern.len() {
            loop {
                let start_char = words.chars().nth(start).unwrap();
                let word_char_freq = *word_freq.get(&start_char).unwrap();
                let pat_char_freq = pat_freq.get(&start_char);
                if pat_char_freq != None && word_char_freq <= *pat_char_freq.unwrap() {
                    break;
                }
                word_freq.entry(start_char).and_modify(|e| *e -= 1);
                start += 1;
            }
            let window_size = i - start + 1;
            if window_size < min_window_size {
                min_window_size = window_size;
                start_index = start;
            }
        }
    }

    if min_window_size > words.len() {
        return String::from("");
    }

    words[start_index..start_index + min_window_size].to_string()
}

fn main() {
    println!("String Window: {}", string_window("hello", "loe"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_pattern() {
        assert_eq!(string_window("hello", "leo"), "ello");
    }

    #[test]
    fn no_containing_chars() {
        assert_eq!(string_window("hello", "xyz"), "");
    }

    #[test]
    fn words_contain_a_single_character_in_pattern() {
        assert_eq!(string_window("hello", "h"), "h");
    }

    #[test]
    fn words_contain_two_window_size() {
        assert_eq!(string_window("helloe", "loe"), "loe");
    }
}
