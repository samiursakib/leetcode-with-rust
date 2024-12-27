use std::collections::HashSet;

fn main() {
    let nums = vec![6,7,8,9];
    let result = Solution::minimum_operations(nums);
    println!("Result: {}", result);
}

struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut store: HashSet<i32> = HashSet::new();
        for (i, &num) in nums.iter().enumerate().rev() {
            if store.contains(&num) {
                return ((i + 1) as f32 / 3.0).ceil() as i32;
            }
            store.insert(num);
        }
        return 0;
    }
}