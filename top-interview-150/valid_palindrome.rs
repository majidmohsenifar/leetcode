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
    for i in 0..only_alphanumeric.len() / 2 {
        if only_alphanumeric[i..i + 1]
            != only_alphanumeric[only_alphanumeric.len() - 1 - i..only_alphanumeric.len() - i]
        {
            return false;
        }
    }

    true
}
