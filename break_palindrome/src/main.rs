fn break_palindrome(input: &str) -> String {
    if input.len() < 2 {
        return String::from("");
    }

    let mut splitted_pal = input
        .split("")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let mid_pos = splitted_pal.len() / 2;
    let odd_pal = splitted_pal.len() % 2 != 0;
    let mut replaced = false;

    for i in 0..splitted_pal.len() {
        if splitted_pal[i] != "a" {
            if odd_pal && i == mid_pos {
                continue;
            }
            splitted_pal[i] = "a";
            replaced = true;
            break;
        }
    }

    if !replaced {
        let last_pos = splitted_pal.len() - 1;
        splitted_pal[last_pos] = "b";
    }

    return splitted_pal.into_iter().collect();
}

fn main() {
    let input = "abccba";
    println!("{:#?}", break_palindrome(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_short_string_length() {
        let input = "a";
        assert_eq!(break_palindrome(&input), "");
    }

    #[test]
    fn two_same_characters() {
        let input = "aa";
        assert_eq!(break_palindrome(&input), "ab");
    }

    #[test]
    fn odd_length_palindrome() {
        let input = "aba";
        assert_eq!(break_palindrome(&input), "abb");
    }

    #[test]
    fn sample_input_palindrome() {
        let input = "abccba";
        assert_eq!(break_palindrome(&input), "aaccba");
    }
}
