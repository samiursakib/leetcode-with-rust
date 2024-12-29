fn main() {
  let grid: Vec<Vec<i32>> = vec![vec![3,2],vec![1,3],vec![3,4],vec![0,1]];
  println!("{}", Solution::minimum_operations(grid)); 
}

struct Solution;

impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let (m, n) = (grid.len(), grid[0].len());
        for j in 0..n {
            let mut end = grid[0][j];
            for i in 1..m {
                let diff = end - grid[i][j];
                end = std::cmp::max(end + 1, grid[i][j]);
                ans += if diff < 0 { 0 } else { diff + 1 };
            }
        }
        ans
    }
}