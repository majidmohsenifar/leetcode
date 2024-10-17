//Write a function that takes the binary representation of a positive integer and returns the number of
//set bits
//it has (also known as the Hamming weight).

//Example 1:
//Input: n = 11
//Output: 3
//Explanation:
//The input binary string 1011 has a total of three set bits.

//Example 2:
//Input: n = 128
//Output: 1
//Explanation:
//The input binary string 10000000 has a total of one set bit.

//Example 3:
//Input: n = 2147483645
//Output: 30
//Explanation:
//The input binary string 1111111111111111111111111111101 has a total of thirty set bits.

fn main() {
    let result = hamming_weight(11);
    println!("result: {:?}", result)
}

pub fn hamming_weight(n: i32) -> i32 {
    let mut res = 0;
    let mut n = n;
    while n > 0 {
        if n % 2 == 1 {
            res += 1;
        }
        n = n / 2;
    }
    res
}
