fn main() {
    let result = word_pattern(String::from("abba"), String::from("dog cat cat fish"));
    println!("result: {}", result)
}

pub fn word_pattern(pattern: String, s: String) -> bool {
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let words: Vec<&str> = s.split(" ").collect();
    if words.len() != pattern_chars.len() {
        return false;
    }
    let mut map: std::collections::HashMap<char, &str> = std::collections::HashMap::new();
    for i in 0..pattern_chars.len() {
        if let Some(word) = map.get(&pattern_chars[i]) {
            if word != &words[i] {
                return false;
            }
        } else {
            for v in map.values() {
                if *v == words[i] {
                    return false;
                }
            }
            map.insert(pattern_chars[i], words[i]);
        }
    }
    true
}
