impl Solution {
  pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let row = triangle.len();
    let col = triangle[row - 1].len();
    let mut dp = vec![0; col + 1];
    let mut temp = [0; 2];
    dp[col] = 22787;
    for i in (0..row).rev() {
      temp[triangle[i].len() % 2] = dp[triangle[i].len()];
      for j in (0..triangle[i].len()).rev() {
        temp[j % 2] = dp[j];
        dp[j] = triangle[i][j] + dp[j].min(temp[(j + 1) % 2]);
      }
    }
    return dp[0];
  }
}

// 进阶答案
pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
  let mut s = 0;
  triangle.iter_mut().for_each(|x| {
    let c = x.iter_mut().rev().next().unwrap();
    *c += s;
    s = *c
  });
  s = 0;
  triangle.iter_mut().for_each(|x| {
    let c = x.iter_mut().next().unwrap();
    *c += s;
    s = *c
  });
  for i in 2..triangle.len() {
    for j in 1..i {
      triangle[i][j] += triangle[i - 1][j].min(triangle[i - 1][j - 1])
    }
  }  triangle.pop().unwrap_or(vec![0]).into_iter().min().unwrap()
}
