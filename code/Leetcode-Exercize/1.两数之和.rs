//https://leetcode-cn.com/problems/two-sum/
/**
 * 运用 BTreeMap 或 HashMap 可以 加快运算速度，contains_key()函数实现时进行了优化
 * 学到的知识
 * 1. 强制类型转换 as type
 * 2. vec.iter().enumerate() 返回 <i,vec[i]>
 * 3. 如何 两个循环 合二为一
 * 4. 用 0..vec.len() 做循环条件
 */
// 笨方法
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut r : Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            for j in 0..nums.len(){
                if(i < j) && (nums[i]+nums[j]==target){
                    r.push(i);
                    r.push(j);
                }
            }
            r
        }
        
    }
}
// 优化方法
use std::collections::BTreeMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: BTreeMap<i32, usize> = BTreeMap::new();
        let mut r: Vec<i32> = vec![-1; 2];
        for (i, &v) in nums.iter().enumerate() {
            let key = &(target - v);
            if map.contains_key(key) {
                let j = *map.get(key).unwrap();
                r[0] = j as i32;
                r[1] = i as i32;
                break;
            }
            map.insert(v, i);
        }
        r
    }
}
//c++ 代码
// #include <iostream>
// #include <vector>
// #include <unordered_map>
// using namespace std;

// class Solution {
// public:
// 	vector<int> twoSum(vector<int>& nums, int target) {
// 		unordered_map<int, int> myhash;
// 		vector<int> out;
// 		for (int i = 0; i < nums.size(); i++) {
// 			myhash[nums[i]] = i;
// 		}
// 		for (int i = 0; i < nums.size(); i++) {
// 			if (myhash[target - nums[i]] && (myhash[target - nums[i]] != i)) {
// 				out.push_back(i);
// 				out.push_back(myhash[target - nums[i]]);
// 				return out;
// 			}
// 		}
// 		return out;
// 	}
// };

// int main()
// {
// 	Solution so;
// 	int arr[4] = {3,3,1,2};
// 	vector<int> arr_(arr, arr + 4);

// 	so.twoSum(arr_, 5);
	
// 	return 0;
// }	

// 作者：wangyuanzhengbighead
// 链接：https://leetcode-cn.com/problems/two-sum/solution/c-rang-ni-neng-li-ke-kan-dong-de-hash-jie-fa-xiao-/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。