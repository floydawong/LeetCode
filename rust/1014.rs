/*

https://leetcode-cn.com/problems/best-sightseeing-pair/

1014. 最佳观光组合

给定正整数数组 A，A[i] 表示第 i 个观光景点的评分，并且两个景点 i 和 j 之间的距离为 j - i。

一对景点（i < j）组成的观光组合的得分为（A[i] + A[j] + i - j）：景点的评分之和减去它们两者之间的距离。

返回一对观光景点能取得的最高分。



示例：

输入：[8,1,5,2,6]
输出：11
解释：i = 0, j = 2, A[i] + A[j] + i - j = 8 + 5 + 0 - 2 = 11


提示：

2 <= A.length <= 50000
1 <= A[i] <= 1000

 */

struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut item_max = a[0];

        for j in 1..a.len() {
            let i = j as i32;
            result = std::cmp::max(result, a[j] - i + item_max);
            item_max = std::cmp::max(item_max, a[j] + i);
        }

        result
    }
}

fn main() {
    let input: Vec<i32> = vec![8, 1, 5, 2, 6];
    let result: i32 = Solution::max_score_sightseeing_pair(input);
    println!("{:?}", result);
}
