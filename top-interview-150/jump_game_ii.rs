//You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].

//Each element nums[i] represents the maximum length of a forward jump from index i. In other words, if you are at nums[i], you can jump to any nums[i + j] where:

//0 <= j <= nums[i] and
//i + j < n
//Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].

//Example 1:
//Input: nums = [2,3,1,1,4]
//Output: 2
//Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
//
//Example 2:
//Input: nums = [2,3,0,1,4]
//Output: 2

fn main() {
    let jumps = vec![2, 3, 1, 1, 4];
    //let jumps = vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0];
    let jump = jump(jumps);
    println!("result is {}", jump);
}

pub fn jump(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut current = 0;
    let mut furthest = 0;

    for i in 0..nums.len() - 1 {
        furthest = furthest.max(i as i32 + nums[i]);
        if i as i32 == current {
            current = furthest;
            count += 1;
        }
    }
    count
}
