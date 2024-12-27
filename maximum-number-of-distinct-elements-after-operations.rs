fn main() {
    let (nums, k) = (vec![9,5,5], 0);
    println!("Result: {}", Solution::max_distinct_elements(nums, k));
}

struct Solution;

impl Solution {
    pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mut last, mut count) = (std::i32::MIN, 0i32);
        for i in 0..nums.len() {
            let (mn, mx) = (nums[i] - k, nums[i] + k);
            if last >= mx {
                count += 1;
            } else {
                last = std::cmp::max(last + 1, mn);
            }
        }
        (nums.len() as i32) - count
    }
}