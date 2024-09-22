//You are climbing a staircase. It takes n steps to reach the top.

//Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

//Example 1:

//Input: n = 2
//Output: 2
//Explanation: There are two ways to climb to the top.
//1. 1 step + 1 step
//2. 2 steps
//Example 2:

//Input: n = 3
//Output: 3
//Explanation: There are three ways to climb to the top.
//1. 1 step + 1 step + 1 step
//2. 1 step + 2 steps
//3. 2 steps + 1 step

fn main() {
    let result = climb_stairs(43);
    println!("result: {}", result)
}

pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    let mut grand_parent = 1;
    let mut parent = 2;
    let mut res = 0;
    for _i in 3..n + 1 {
        res = parent + grand_parent;
        grand_parent = parent;
        parent = res;
    }
    res
    ///recursive did not work because of timeout
    //return climb_stairs(n - 1) + climb_stairs(n - 2);
}
