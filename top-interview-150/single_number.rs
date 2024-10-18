//Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.

//You must implement a solution with a linear runtime complexity and use only constant extra space.

//Example 1:
//Input: nums = [2,2,1]
//Output: 1

//Example 2:
//Input: nums = [4,1,2,1,2]
//Output: 4
//
//Example 3:
//Input: nums = [1]
//Output: 1
fn main() {
    let result = single_number(Vec::from([2, 2, 1]));
    println!("result: {:?}", result)
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    //When a number is XORed with itself, the result is 0.
    //And when a number is XORed with 0, the result is the number itself.
    //So, if we XOR all the numbers in the array, all the numbers appearing twice will cancel each other out
    //and we'll be left with the single number that appears only once.
    let mut res = 0;
    for i in 0..nums.len() {
        res ^= nums[i];
    }
    res
}
