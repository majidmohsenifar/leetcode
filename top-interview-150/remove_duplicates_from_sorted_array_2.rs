fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let k = remove_duplicates(&mut nums);
    println!("k:{}, nums: {:?}", k, nums);
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }
    let mut write_idx: i32 = 2;
    let mut val_to_compare = nums[0];
    for i in 2..nums.len() {
        if nums[i] != val_to_compare {
            val_to_compare = nums[i - 1];
            nums[write_idx as usize] = nums[i];
            write_idx += 1;
        }
    }
    write_idx
}
