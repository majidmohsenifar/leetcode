//Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

//An input string is valid if:

//Open brackets must be closed by the same type of brackets.
//Open brackets must be closed in the correct order.
//Every close bracket has a corresponding open bracket of the same type.

//Example 1:

//Input: s = "()"

//Output: true

//Example 2:

//Input: s = "()[]{}"

//Output: true

//Example 3:

//Input: s = "(]"

//Output: false

//Example 4:

//Input: s = "([])"

//Output: true

fn main() {
    let result = is_valid("()".to_string());
    println!("result: {}", result)
}

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        if let Some(c) = stack.pop() {
            match (c, ch) {
                ('(', ')') => continue,
                ('{', '}') => continue,
                ('[', ']') => continue,
                _ => {
                    stack.push(c);
                    stack.push(ch);
                }
            }
        } else {
            stack.push(ch);
        }
    }
    return stack.len() == 0;
}
