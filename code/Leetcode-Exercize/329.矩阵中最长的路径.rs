/**
 * https://leetcode-cn.com/problems/longest-increasing-path-in-a-matrix/
 * 
 * 329. 矩阵中的最长递增路径
 * 
 * 给定一个整数矩阵，找出最长递增路径的长度。

对于每个单元格，你可以往上，下，左，右四个方向移动。 你不能在对角线方向上移动或移动到边界外（即不允许环绕）。

示例 1:

输入: nums = 
[
  [9,9,4],
  [6,6,8],
  [2,1,1]
] 
输出: 4 
解释: 最长递增路径为 [1, 2, 6, 9]。
示例 2:

输入: nums = 
[
  [3,4,5],
  [3,2,6],
  [2,2,1]
] 
输出: 4 
解释: 最长递增路径是 [3, 4, 5, 6]。注意不允许在对角线方向上移动。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/longest-increasing-path-in-a-matrix
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

 
#![inline]
fn check(matrix:&[Vec<i32>],i:usize,j:usize)->i32{
    if let Some(r)=matrix.get(i){if let Some(x)=r.get(j){*x}else{0}}else{0}
}

struct Solution;
impl Solution {
    pub fn longest_increasing_path(mut matrix: Vec<Vec<i32>>) -> i32 {
        let mut heap=std::collections::BinaryHeap::new();
        for (i,r) in matrix.iter().enumerate(){
            for (j,x) in r.iter().copied().enumerate(){
                heap.push(std::cmp::Reverse((x,i,j)));
            }
        }
        let mut cur=-2147483648;
        let mut array=Vec::new();
        while let Some(std::cmp::Reverse(x))=heap.pop(){
            let(i,j)=(x.1,x.2);
            if cur!=x.0{array.drain(..).for_each(|(x,i,j):(i32,usize,usize)|matrix[i][j]=x);cur=x.0;}
            array.push((check(&matrix,i-1,j).min(check(&matrix,i+1,j)).min(check(&matrix,i,j+1)).min(check(&matrix,i,j-1)).min(0)-1,i,j));
        }
        array.drain(..).for_each(|(x,i,j):(i32,usize,usize)|matrix[i][j]=x);
        -matrix.drain(..).map(|mut i|i.drain(..).min().unwrap_or(0)).min().unwrap_or(0)
    }
}

fn main() {
  let mut a = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
  let r = Solution::longest_increasing_path(a);
  println!("{:?}", r);
}
