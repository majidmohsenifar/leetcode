fn main() {
    let mut nums = vec![1, 1, 2];
    let k = remove_duplicates(&mut nums);
    println!("k:{}, nums: {:?}", k, nums);
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut write_idx: i32 = 1;
    let len = nums.len();
    let mut j = 1;
    while j < len {
        for i in j..len {
            if nums[i] != nums[write_idx as usize - 1] {
                nums[write_idx as usize] = nums[i];
                write_idx += 1;
            }
            j += 1;
        }
    }
    write_idx
}
