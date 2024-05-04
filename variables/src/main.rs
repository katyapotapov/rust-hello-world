fn count_vowels(word: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
        word.to_lowercase().chars().filter(|&c| vowels.contains(&c)).count()
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("didnt work");

    let word = first_word(&s);
    
    let stats = WordStats {
        word,
        is_elephant: word == "elephant",
        num_vowels: count_vowels(word)
    };

    println!("{}, is elephant: {}, num vowels: {}", word, stats.is_elephant, stats.num_vowels);

}

struct WordStats {
    word: &str,
    is_elephant: bool,
    num_vowels: usize,
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
