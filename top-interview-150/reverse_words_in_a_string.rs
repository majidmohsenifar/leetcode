//Given an input string s, reverse the order of the words.

//A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.

//Return a string of the words in reverse order concatenated by a single space.

//Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.

//Example 1:

//Input: s = "the sky is blue"
//Output: "blue is sky the"
//Example 2:

//Input: s = "  hello world  "
//Output: "world hello"
//Explanation: Your reversed string should not contain leading or trailing spaces.
//Example 3:

//Input: s = "a good   example"
//Output: "example good a"
//Explanation: You need to reduce multiple spaces between two words to a single space in the reversed string.

fn main() {
    let result = reverse_words(String::from("the sky is blue"));
    //let result = reverse_words(String::from("  hello world  "));
    println!("result: {}", result)
}

pub fn reverse_words(s: String) -> String {
    let mut without_extra_spaces = String::new();
    let mut word = String::new();
    for ch in s.chars() {
        if ch != ' ' {
            word.push(ch);
        } else {
            if word.len() == 0 {
                continue;
            }
            without_extra_spaces.push_str(&word);
            without_extra_spaces.push(' ');
            word.clear();
        }
    }
    if word.len() > 0 {
        without_extra_spaces.push_str(&word);
    }
    if let Some(ch) = without_extra_spaces.chars().last() {
        if ch == ' ' {
            without_extra_spaces.pop();
        }
    }
    let mut result = String::new();
    let words: Vec<&str> = without_extra_spaces.split(' ').collect();
    for (i, word) in words.iter().rev().enumerate() {
        result.push_str(&word);
        if i != words.len() - 1 {
            result.push_str(" ");
        }
    }
    result
}
