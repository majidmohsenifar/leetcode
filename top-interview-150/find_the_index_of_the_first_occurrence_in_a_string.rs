fn main() {
    //let result = str_str(String::from("sadbutsad"), String::from("sad"));
    let result = str_str(String::from("a"), String::from("a"));
    println!("result: {}", result)
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let needle_len = needle.len();
    if needle_len > haystack.len() {
        return -1;
    }
    for i in 0..haystack.len() - needle_len + 1 {
        if haystack[i..i + needle.len()] == needle {
            return i as i32;
        }
    }
    -1
}
