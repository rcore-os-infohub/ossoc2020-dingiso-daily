struct Solution;
impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut rec = vec![vec![0; n + 2]; n + 2];
        let val = (1..=1)
            .chain(nums.drain(..))
            .chain(1..=1)
            .collect::<Vec<i32>>();
        for i in (0..n).rev() {
            for j in (i + 2)..(n + 2) {
                for k in (i + 1)..j {
                    rec[i][j] = rec[i][j].max(val[i] * val[k] * val[j] + rec[i][k] + rec[k][j])
                }
            }
        }
        rec[0][n + 1]
    }
}

fn main() {}
