fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let sentence = "This is a test sentence";
    if let Some(shortest) = shortest_word(sentence) {
        println!("Shortest word in the sentence: {}", shortest);
    } else {
        println!("No words found in the sentence.");
    }
}
