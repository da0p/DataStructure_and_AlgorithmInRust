fn length_encode(input: &str) -> String {
    let splitted_input = input
        .split("")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let mut temp = splitted_input[0];
    let mut counter = 1;
    let mut encoded_input = String::from("");

    for i in 1..splitted_input.len() {
        if counter == 1 {
            encoded_input += temp;
        }
        if splitted_input[i] == temp {
            counter += 1;
        } else {
            encoded_input += &counter.to_string();
            counter = 1;
        }
        temp = splitted_input[i];
    }
    if counter > 1 {
        encoded_input += &counter.to_string();
    } else {
        encoded_input += temp;
    }

    if encoded_input.len() > input.len() {
        return input.to_owned();
    }

    encoded_input
}

fn main() {
    let input = "bbbaaaadexxxxxx";
    println!("{}", length_encode(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_all_equal_character() {
        let input = "aaaa";
        assert_eq!(length_encode(&input), "a4");
    }

    #[test]
    fn encode_two_set_of_characters() {
        let input = "aaaabbbb";
        assert_eq!(length_encode(&input), "a4b4");
    }

    #[test]
    fn encode_different_characters() {
        let input = "bbbaaaadexxxxxx";
        assert_eq!(length_encode(&input), "b3a4d1e1x6");
    }

    #[test]
    fn encode_one_character_each() {
        let input = "abc";
        assert_eq!(length_encode(&input), "abc");
    }
}
