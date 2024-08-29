fn main() {
    let jumps = vec![3, 2, 1, 0, 4];
    //let jumps = vec![0];
    let can_jump = can_jump(jumps);
    println!("can_jump: {}", can_jump);
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }
    let max = nums.len() as i32 - 1;
    for i in 0..nums.len() - 1 {
        if max - i as i32 <= nums[i] {
            return true;
        }
    }
    false

    //let mut max_gain_power = nums[0]; //1
    //for i in 0..nums.len() {
    //if max_jump_count <= max_gain_power {
    //return true;
    //}
    //let mut j = 0;
    //while j <= nums[i] {
    //let temp_gain_power = j + nums[j as usize];
    //if temp_gain_power >= max_gain_power {
    //max_gain_power = temp_gain_power;
    //max_jump_count = max_jump_count - max_gain_power;
    //}
    //j += 1;
    //}
    //}
    //false
}
