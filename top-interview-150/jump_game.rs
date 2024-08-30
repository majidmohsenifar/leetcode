fn main() {
    let jumps = vec![3, 0, 8, 2, 0, 0, 1];
    let can_jump = can_jump(jumps);
    println!("can_jump: {}", can_jump);
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }
    let mut jump_needed = nums.len() as i32 - 1;
    let mut jump_power = 0;
    for i in 0..nums.len() - 1 {
        if nums[i] >= jump_power {
            jump_power = nums[i];
            if jump_power >= jump_needed {
                return true;
            }
        }
        jump_needed -= 1;
        jump_power -= 1;
        if jump_power < 0 {
            return false;
        }
    }
    false
}
