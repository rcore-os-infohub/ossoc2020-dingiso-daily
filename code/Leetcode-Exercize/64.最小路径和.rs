struct Solution;
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (row, col) = (grid.len(), grid[0].len());
        let mut dp: Vec<i32> = vec![99999; col + 1];
        dp[col - 1] = 0;
        for i in (0..row).rev() {
            for j in (0..col).rev() {
                dp[j] = grid[i][j] + dp[j].min(dp[j + 1]);
            }
        }
        return dp[0];
    }
}
