/// 'search_all' finds all starting indices of a key word in a string
/// 
fn search_all(key: &str, input: &str) -> Vec<usize> {
    return input.match_indices(key).map(|(i, _)| i).collect::<Vec<_>>();
}

fn main() {
    println!(
        "All indices: {:#?}",
        search_all("movie", "I liked the movie, acting in movie was great!")
    );
}
