fn space20(input: &str) -> String
{
    input.replace(" ", r"%20")
}

fn main() {
    println!("{}", space20("hello world, how are you?"));
}
