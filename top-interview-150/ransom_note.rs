fn main() {
    let result = can_construct(
        String::from("bg"),
        String::from("efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj"),
    );
    println!("result: {}", result)
}

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let magazine_chars: Vec<char> = magazine.chars().collect();
    let ransom_chars: Vec<char> = ransom_note.chars().collect();

    let mut map = std::collections::HashMap::new();
    for i in 0..ransom_chars.len() {
        map.entry(ransom_chars[i])
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    for i in 0..magazine_chars.len() {
        map.entry(magazine_chars[i]).and_modify(|c| *c -= 1);
    }
    for v in map.values() {
        if *v > 0 {
            return false;
        }
    }

    true
}
