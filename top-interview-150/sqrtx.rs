//Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.

//You must not use any built-in exponent function or operator.

//For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.

//Example 1:

//Input: x = 4
//Output: 2
//Explanation: The square root of 4 is 2, so we return 2.
//Example 2:

//Input: x = 8
//Output: 2
//Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.

fn main() {
    //let result = my_sqrt(28);
    let result = my_sqrt(6);
    //let result = my_sqrt(2147395599);
    println!("result: {:#?}", result)
}

pub fn my_sqrt(x: i32) -> i32 {
    if (0..2).contains(&x) {
        return x;
    }
    let mut res = x / 2;
    loop {
        let mul = res.checked_mul(res);
        if mul.is_some() && mul <= Some(x) {
            break;
        }
        res = res / 2;
        if res == 0 {
            res = 1;
        }
    }
    for i in (res..res * 2 + 1).rev() {
        let mul = i.checked_mul(i);
        if mul.is_some() && mul <= Some(x) {
            return i;
        }
    }
    res
}
