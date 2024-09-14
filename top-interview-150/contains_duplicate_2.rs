//Given an integer array nums and an integer k,
//return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.

//Example 1:

//Input: nums = [1,2,3,1], k = 3
//Output: true
//Example 2:

//Input: nums = [1,0,1,1], k = 1
//Output: true
//Example 3:

//Input: nums = [1,2,3,1,2,3], k = 2
//Output: false

fn main() {
    let result = contains_nearby_duplicate(vec![1, 2, 3, 1], 3);
    println!("result: {}", result)
}

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    for (i, &v) in nums.iter().enumerate() {
        if let Some(&i2) = map.get(&v) {
            if i2 != i && (i2 as i32 - i as i32).abs() <= k {
                return true;
            }
        }
        map.insert(v, i);
    }
    return false;
}
