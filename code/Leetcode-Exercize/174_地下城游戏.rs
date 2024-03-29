/**
 * 动态规划习题
 * leetcode 174
 * 一些恶魔抓住了公主（P）并将她关在了地下城的右下角。地下城是由 M x N 个房间组成的二维网格。我们英勇的骑士（K）最初被安置在左上角的房间里，他必须穿过地下城并通过对抗恶魔来拯救公主。

骑士的初始健康点数为一个正整数。如果他的健康点数在某一时刻降至 0 或以下，他会立即死亡。

有些房间由恶魔守卫，因此骑士在进入这些房间时会失去健康点数（若房间里的值为负整数，则表示骑士将损失健康点数）；其他房间要么是空的（房间里的值为 0），要么包含增加骑士健康点数的魔法球（若房间里的值为正整数，则表示骑士将增加健康点数）。

为了尽快到达公主，骑士决定每次只向右或向下移动一步。



编写一个函数来计算确保骑士能够拯救到公主所需的最低初始健康点数。

例如，考虑到如下布局的地下城，如果骑士遵循最佳路径 右 -> 右 -> 下 -> 下，则骑士的初始健康点数至少为 7。

-2 (K)	-3	3
-5	-10	1
10	30	-5 (P)


说明:

骑士的健康点数没有上限。

任何房间都可能对骑士的健康点数造成威胁，也可能增加骑士的健康点数，包括骑士进入的左上角房间以及公主被监禁的右下角房间。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/dungeon-game
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */
struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (row, col) = (dungeon.len(), dungeon[0].len());
        let mut dp = vec![0; row * col];
        for i in (0..row).rev() {
            for j in (0..col).rev() {
                if (i == row - 1) && (j == col - 1) {
                    dp[i * col + j] = 1.max(1 - dungeon[i][j]);
                } else if i == row - 1 {
                    dp[i * col + j] = 1.max(dp[i * col + j + 1] - dungeon[i][j]);
                } else if j == col - 1 {
                    dp[i * col + j] = 1.max(dp[i * col + j + col] - dungeon[i][j]);
                } else {
                    dp[i * col + j] =
                        1.max(dp[i * col + j + 1].min(dp[i * col + j + col]) - dungeon[i][j]);
                }
            }
        }
        return dp[0];
    }
}

fn main() {}

//c++ 代码
// class Solution {
//     public:
//         int calculateMinimumHP(vector<vector<int>>& dungeon) {
//             int m = dungeon.size(), n = dungeon[0].size();
//             if(m == 0 || n == 0)
//                 return 0;
//             // dp[i][j] 表示骑士在此位置所需要的最小生命值
//             vector<vector<int>> dp(m + 1, vector<int> (n + 1, INT_MAX));
//             // 初始化
//             dp[m][n - 1] = dp[m - 1][n] = 1;
//             for(int i = m - 1; i >= 0; i--)
//             {
//                 for(int j = n - 1; j >= 0; j--)
//                 {
//                     dp[i][j] = max(min(dp[i + 1][j], dp[i][j + 1]) - dungeon[i][j], 1);
//                 }
//             }
//             return dp[0][0];
//         }
//     };
//     作者：Sunny_SMILE
//     链接：https://leetcode-cn.com/problems/dungeon-game/solution/174-di-xia-cheng-you-xi-er-wei-dp-yi-wei-dp-by-sun/
//     来源：力扣（LeetCode）
//     著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
