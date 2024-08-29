fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut nums, 3);
    println!("nums: {:?}", nums);
}

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len() - 1;
    for _i in 0..k {
        let last_element = nums[nums.len() - 1];
        for j in (0..len).rev() {
            nums[j + 1] = nums[j];
        }
        nums[0] = last_element;
    }
}
