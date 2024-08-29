fn main() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let majority = majority_element(nums);
    println!("majority:{}", majority);
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut majority = 0;
    let mut vote = 0;
    for num in nums {
        if vote == 0 {
            majority = num;
        }
        if num == majority {
            vote += 1;
        } else {
            vote -= 1;
        }
    }
    majority
}
