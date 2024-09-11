fn main() {
    let input = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let result = longest_common_prefix(input);
    println!("result: {}", result)
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut res: Vec<char> = strs[0].chars().collect();
    for word in &strs[1..] {
        if word.len() < res.len() {
            res = Vec::from(&res[0..word.len()]);
        }
        for (i, char) in word.chars().enumerate() {
            if i < res.len() && res[i] != char {
                res = Vec::from(&res[0..i]);
            }
        }
    }
    res.iter().collect()
}
