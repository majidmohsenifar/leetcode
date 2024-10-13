//Given an integer x, return true if x is a
//palindrome
//, and false otherwise.

//Example 1:

//Input: x = 121
//Output: true
//Explanation: 121 reads as 121 from left to right and from right to left.
//Example 2:

//Input: x = -121
//Output: false
//Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//Example 3:

//Input: x = 10
//Output: false
//Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

fn main() {
    let result = is_palindrome(121);
    println!("result: {}", result)
}

pub fn is_palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    }
    let mut digits = vec![];
    let mut temp = x;
    while temp > 0 {
        let digit = temp % 10;
        digits.push(digit);
        temp = temp / 10;
    }
    for i in 0..digits.len() / 2 {
        if digits[i] != digits[digits.len() - 1 - i] {
            return false;
        }
    }
    true
}
