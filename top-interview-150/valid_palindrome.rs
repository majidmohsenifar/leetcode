fn main() {
    let result = is_palindrome(String::from("A man, a plan, a canal: Panama"));

    println!("result: {}", result)
}

pub fn is_palindrome(s: String) -> bool {
    //first we remove all non alpha-numeric characters
    let mut only_alphanumeric = String::new();
    for ch in s.to_lowercase().chars() {
        if ch.is_alphanumeric() {
            only_alphanumeric.push(ch);
        }
    }

    let v: Vec<char> = only_alphanumeric.chars().collect();
    for i in 0..v.len() / 2 {
        if v[i] != v[v.len() - i - 1] {
            return false;
        }
    }

    true
}
