fn main() {
    let string = String::from("something was not correct ");
    let result = length_of_last_word(string);
    println!("result: {}", result)
}

pub fn length_of_last_word(s: String) -> i32 {
    let mut res = 0;
    for ch in s.chars().rev() {
        if ch != ' ' {
            res += 1;
        } else if res > 0 {
            break;
        }
    }
    return res;
}
