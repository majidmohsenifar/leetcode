fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let expected = remove_element(&mut nums, val);
    println!("expected:{}, nums:{:?}", expected, nums)
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut write_idx = 0;
    let len = nums.len();
    for i in 0..len {
        if nums[i] != val {
            nums[write_idx as usize] = nums[i];
            write_idx += 1;
        }
    }
    write_idx
}
