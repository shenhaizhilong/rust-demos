use std::cmp::Reverse;
use std::collections::BinaryHeap;
use crate::binary_tree::BinaryTree;


/**
215. 数组中的第K个最大元素


题目描述
给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。

请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

你必须设计并实现时间复杂度为 O(n) 的算法解决此问题。



示例 1:

输入: [3,2,1,5,6,4], k = 2
输出: 5
示例 2:

输入: [3,2,3,1,2,4,5,5,6], k = 4
输出: 4


提示：

1 <= k <= nums.length <= 105
-104 <= nums[i] <= 104
解法
方法一：快速选择
快速选择算法是一种在未排序的数组中查找第 k 个最大元素或最小元素的算法。它的基本思想是每次选择一个基准元素，将数组分为两部分，一部分的元素都比基准元素小，另一部分的元素都比基准元素大，然后根据基准元素的位置，决定继续在左边还是右边查找，直到找到第 k 个最大元素。
时间复杂度
O(n)，空间复杂度
O(logn)。其中
n 为数组
nums 的长度。
**/
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    // 最大堆
    let mut heap = BinaryHeap::new();
    for &x in nums.iter() {
        heap.push(-x);
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    return -heap.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::find_kth_largest::find_kth_largest;

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 1], 2), 2);
        assert_eq!(find_kth_largest(vec![3,2,1,5,6,4], 2), 5);
        assert_eq!(find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4), 4);
    }
}