fn main() {
    let result = is_anagram(String::from("anagram"), String::from("nagaram"));
    println!("result: {}", result)
}

pub fn is_anagram(s: String, t: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    if s_chars.len() != t_chars.len() {
        return false;
    }
    let mut map = std::collections::HashMap::new();
    for i in 0..s_chars.len() {
        map.entry(&s_chars[i]).and_modify(|c| *c += 1).or_insert(1);
    }
    for i in 0..t_chars.len() {
        map.entry(&t_chars[i]).and_modify(|c| *c -= 1);
        //println!("{},{}", t_chars[i], count);
        //if *count != 0 {
        //return false;
        //}
    }
    for v in map.values() {
        if *v != 0 {
            return false;
        }
    }
    return true;
}
