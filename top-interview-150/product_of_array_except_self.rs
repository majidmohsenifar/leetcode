//Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

//The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

//You must write an algorithm that runs in O(n) time and without using the division operation.

//Example 1:
//Input: nums = [1,2,3,4]
//Output: [24,12,8,6]
//
//Example 2:
//Input: nums = [-1,1,0,-3,3]
//Output: [0,0,9,0,0]

fn main() {
    //let result = product_except_self(vec![1, 2, 3, 4]);
    //let result = product_except_self(vec![-1, 1, 0, -3, 3]);
    let result = product_except_self(vec![4, 3, 2, 1, 2]);
    println!("result: {:?}", result)
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![1; nums.len()];
    let mut p: i32 = 1;
    for i in 1..nums.len() {
        p *= nums[i - 1];
        res[i] = p;
    }
    let mut t: i32 = 1;
    for i in (0..nums.len() - 1).rev() {
        t *= nums[i + 1];
        res[i] *= t;
    }
    res
}
