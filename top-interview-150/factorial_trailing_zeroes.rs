//Given an integer n, return the number of trailing zeroes in n!.

//Note that n! = n * (n - 1) * (n - 2) * ... * 3 * 2 * 1.

//Example 1:

//Input: n = 3
//Output: 0
//Explanation: 3! = 6, no trailing zero.
//Example 2:

//Input: n = 5
//Output: 1
//Explanation: 5! = 120, one trailing zero.
//Example 3:

//Input: n = 0
//Output: 0

fn main() {
    let result = trailing_zeroes(25);
    println!("result: {:#?}", result)
}

pub fn trailing_zeroes(n: i32) -> i32 {
    let mut number_of_5 = 0;
    for i in (5..n + 1).step_by(5) {
        let mut temp = i;
        while temp % 5 == 0 {
            number_of_5 += 1;
            temp = temp / 5;
        }
    }
    number_of_5
}
