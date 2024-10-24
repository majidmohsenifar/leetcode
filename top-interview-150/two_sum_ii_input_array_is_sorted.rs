//Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.

//Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.

//The tests are generated such that there is exactly one solution. You may not use the same element twice.

//Your solution must use only constant extra space.

//Example 1:
//Input: numbers = [2,7,11,15], target = 9
//Output: [1,2]
//Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].

//Example 2:
//Input: numbers = [2,3,4], target = 6
//Output: [1,3]
//Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].

//Example 3:
//Input: numbers = [-1,0], target = -1
//Output: [1,2]
//Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
fn main() {
    //let result = two_sum(vec![2, 7, 11, 15], 9);
    let result = two_sum2(vec![5, 25, 75], 100);
    println!("result: {:?}", result)
}

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut p1: i32 = 0;
    let mut p2: i32 = 1;
    let len = numbers.len();
    while p1 < len as i32 && p2 < len as i32 {
        let sum = numbers[p1 as usize] + numbers[p2 as usize];
        match (sum).cmp(&target) {
            std::cmp::Ordering::Less => {
                if p2 + 1 < len as i32 {
                    p2 += 1;
                } else {
                    p1 += 1;
                    p2 = p1 + 1;
                }
            }
            std::cmp::Ordering::Greater => {
                p1 += 1;
                p2 = p1 + 1;
            }
            std::cmp::Ordering::Equal => {
                return vec![p1 + 1, p2 + 1];
            }
        }
    }
    vec![]
}

pub fn two_sum2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut p1: i32 = 0;
    let mut p2: i32 = numbers.len() as i32 - 1;
    while p1 < p2 {
        let sum = numbers[p1 as usize] + numbers[p2 as usize];
        match (sum).cmp(&target) {
            std::cmp::Ordering::Less => p1 += 1,
            std::cmp::Ordering::Greater => p2 -= 1,
            std::cmp::Ordering::Equal => {
                return vec![p1 + 1, p2 + 1];
            }
        }
    }
    vec![]
}
