
struct Solution;
impl Solution {
  pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let chars1 = s1.as_bytes();
    let chars2 = s2.as_bytes();
    let chars3 = s3.as_bytes();
    let len1 = s1.len();
    let len2 = s2.len();
    let len3 = s3.len();
    let mut p = 0;
    if len1 + len2 != len3 {
      return false;
    }
    let mut dp: Vec<Vec<bool>> = vec![vec![false; len2 + 1]; len1 + 1];
    dp[0][0] = true;
    for i in 0..=len1 {
      for j in 0..=len2 {
        if (i + j) > 0 {
          p = i + j - 1;
        }
        if i > 0 {
          dp[i][j] = dp[i][j] || (dp[i - 1][j] && chars1[i - 1] == chars3[p]);
        }
        if j > 0 {
          dp[i][j] = dp[i][j] || (dp[i][j - 1] && chars2[j - 1] == chars3[p]);
        }
      }
    }
    return dp[len1][len2];
  }
}

fn main() {
  let s1 = String::from("a");
  let s2 = String::from("");
  let s3 = String::from("a");
  println!("{}", Solution::is_interleave(s1, s2, s3));
}
