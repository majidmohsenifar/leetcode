//Write an algorithm to determine if a number n is happy.

//A happy number is a number defined by the following process:

//Starting with any positive integer, replace the number by the sum of the squares of its digits.
//Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
//Those numbers for which this process ends in 1 are happy.
//Return true if n is a happy number, and false if not.

fn main() {
    let result = is_happy(19);
    println!("result: {}", result)
}

pub fn is_happy(n: i32) -> bool {
    let mut n = n;
    let mut sum = 0;
    let mut set: std::collections::HashSet<i32> = std::collections::HashSet::new();
    let mut digit = 0;
    loop {
        digit = n % 10;
        sum += digit.pow(2);
        n = n / 10;
        if n == 0 {
            if sum == 1 {
                return true;
            }
            if let Some(k) = set.get(&sum) {
                return false;
                break;
            } else {
                set.insert(sum);
            }
            n = sum;
            sum = 0;
        }
    }

    false
}
