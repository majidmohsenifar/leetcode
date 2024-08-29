fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;
    merge(&mut nums1, m, &mut nums2, n);
    println!("nums1: {:?}", nums1)
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut p1 = m - 1;
    let mut p2 = n - 1;
    let mut p = m + n - 1;
    while p > -1 {
        if p1 < 0 {
            nums1[p as usize] = nums2[p2 as usize];
            p -= 1;
            p2 -= 1;
        } else if p2 < 0 {
            nums1[p as usize] = nums1[p1 as usize];
            p -= 1;
            p1 -= 1;
        } else if nums1[p1 as usize] > nums2[p2 as usize] {
            nums1[p as usize] = nums1[p1 as usize];
            p -= 1;
            p1 -= 1;
        } else {
            nums1[p as usize] = nums2[p2 as usize];
            p -= 1;
            p2 -= 1;
        }
    }
}
