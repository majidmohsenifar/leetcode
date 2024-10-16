//Given two binary strings a and b, return their sum as a binary string.

//Example 1:
//Input: a = "11", b = "1"
//Output: "100"
//
//Example 2:
//Input: a = "1010", b = "1011"
//Output: "10101"

fn main() {
    let result = add_binary("101111".to_string(), "10".to_string());
    println!("result: {:?}", result)
}

pub fn add_binary(a: String, b: String) -> String {
    let mut a_chars: Vec<char> = a.chars().collect();
    let mut b_chars: Vec<char> = b.chars().collect();
    let mut has_remainder = false;
    let mut res = String::new();
    let mut loop_count = a_chars.len();
    if b_chars.len() > loop_count {
        loop_count = b_chars.len();
    }
    for _i in 0..loop_count {
        match (a_chars.pop(), b_chars.pop()) {
            (Some(a), Some(b)) => {
                if a == '1' && b == '1' {
                    if has_remainder {
                        res.push('1');
                    } else {
                        res.push('0');
                    }
                    has_remainder = true;
                    continue;
                }
                if a == '1' || b == '1' {
                    if has_remainder {
                        res.push('0');
                        has_remainder = true;
                    } else {
                        res.push('1');
                    }
                    continue;
                }
                if a == '0' && b == '0' {
                    if has_remainder {
                        res.push('1');
                        has_remainder = false;
                    } else {
                        res.push('0');
                    }
                }
                //if a == '0' || b == '0' {
                //if has_remainder {
                //res.push('1');
                //has_remainder = false;
                //} else {
                //res.push('0');
                //}
                //}
            }
            (Some(ch), None) | (None, Some(ch)) => {
                if ch == '1' {
                    if has_remainder {
                        res.push('0');
                        has_remainder = true;
                    } else {
                        res.push('1');
                    }
                } else {
                    if has_remainder {
                        res.push('1');
                        has_remainder = false;
                    } else {
                        res.push('0');
                    }
                }
            }
            (None, None) => {}
        }
    }
    if has_remainder {
        res.push('1');
    }
    let str: String = res
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .into_iter()
        .collect();
    str
}
