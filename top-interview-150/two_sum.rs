fn main() {
    let result = two_sum(vec![2, 7, 11, 15], 9);
    println!("result: {:?}", result)
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        map.insert(*v, i);
    }
    for (i, v) in nums.iter().enumerate() {
        if let Some(i2) = map.get(&(target - v)) {
            if i != *i2 {
                return vec![i as i32, *i2 as i32];
            }
        }
    }
    return vec![];
}
