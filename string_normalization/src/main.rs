fn normalize_string(input: &str) -> String {
    let splitted_input = input.split(" ").collect::<Vec<_>>();
    splitted_input
        .into_iter()
        .map(|w| {
            let mut new_word = String::from("");
            new_word += &w.chars().next().unwrap().to_ascii_uppercase().to_string();
            new_word += &w[1..].to_ascii_lowercase();
            new_word
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let input = "This is SO MUCH FUN!";
    println!("{:#?}", normalize_string(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "This is SO MUCH FUN!";
        assert_eq!(normalize_string(&input), "This Is So Much Fun!");
    }
}