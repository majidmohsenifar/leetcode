fn main() {
    let result = is_isomorphic(String::from("badc"), String::from("baba"));
    println!("result: {}", result)
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let mut map = std::collections::HashMap::new();
    for i in 0..s_chars.len() {
        if let Some(ch) = map.get(&s_chars[i]) {
            if *ch != t_chars[i] {
                return false;
            }
        } else {
            for v in map.values() {
                if *v == t_chars[i] {
                    return false;
                }
            }
            map.insert(s_chars[i], t_chars[i]);
        }
    }
    true
}
