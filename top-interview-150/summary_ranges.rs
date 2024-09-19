//You are given a sorted unique integer array nums.

//A range [a,b] is the set of all integers from a to b (inclusive).

//Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.

//Each range [a,b] in the list should be output as:

//"a->b" if a != b
//"a" if a == b

//Example 1:

//Input: nums = [0,1,2,4,5,7]
//Output: ["0->2","4->5","7"]
//Explanation: The ranges are:
//[0,2] --> "0->2"
//[4,5] --> "4->5"
//[7,7] --> "7"
//Example 2:

//Input: nums = [0,2,3,4,6,8,9]
//Output: ["0","2->4","6","8->9"]
//Explanation: The ranges are:
//[0,0] --> "0"
//[2,4] --> "2->4"
//[6,6] --> "6"
//[8,9] --> "8->9"

fn main() {
    //TODO:
    //let result = summary_ranges(vec![0, 1, 2, 4, 5, 7]);
    let result = summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]);
    println!("result: {:#?}", result)
}

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut range: Vec<i32> = Vec::new();
    if nums.len() == 0 {
        return result;
    }
    range.push(nums[0]);
    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] + 1 {
            range.push(nums[i])
        } else {
            if range.len() > 1 {
                result.push(range[0].to_string() + "->" + &range[range.len() - 1].to_string());
            } else {
                result.push(range[0].to_string());
            }
            range.clear();
            range.push(nums[i]);
        }
    }
    if range.len() > 1 {
        result.push(range[0].to_string() + "->" + &range[range.len() - 1].to_string());
    } else {
        result.push(range[0].to_string());
    }
    result
}
