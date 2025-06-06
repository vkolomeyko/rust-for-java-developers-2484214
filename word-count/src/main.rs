use std::collections::HashMap;

static TEXT: &str = "
hello, world!
how are you?
i'm good, how are you?
i'm good too. i hope that you are doing well.
";

fn tokenize(text: &str) -> impl Iterator<Item = &str> {
    // Split input text on whitespace
    text.split_whitespace()
        // Strip punctuation from tokens
        .map(|x| x.trim_matches(|c: char| c.is_ascii_punctuation()))
        // Set minimum length of 2
        .filter(|x| x.len() > 2)
}

fn main() {
    let word_counts = tokenize(TEXT).fold(HashMap::new(), |mut map: HashMap<&str, u32>, word| {
        let new_value = match map.get(word) {
            Some(value) => value + 1,
            None => 1
        };
        map.insert(word, new_value);
        map
    });

    for (word, count) in word_counts.iter() {
        println!("{}: {}", word, count);
    }
}
