fn main() {
    let result = is_subsequence(String::from(""), String::from("abc"));
    println!("result: {}", result)
}

pub fn is_subsequence(s: String, t: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    let mut j: usize = 0;
    for i in 0..t_chars.len() {
        if s_chars.len() > 0 && t_chars[i] == s_chars[j] {
            j += 1;
        }
        if j == s.len() {
            return true;
        }
    }
    j == s.len()
}
