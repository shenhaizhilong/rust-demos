/**
26. 删除有序数组中的重复项


题目描述
给你一个 非严格递增排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。然后返回 nums 中唯一元素的个数。

考虑 nums 的唯一元素的数量为 k ，你需要做以下事情确保你的题解可以被通过：

更改数组 nums ，使 nums 的前 k 个元素包含唯一元素，并按照它们最初在 nums 中出现的顺序排列。nums 的其余元素与 nums 的大小不重要。
返回 k 。
判题标准:

系统会用下面的代码来测试你的题解:

int[] nums = [...]; // 输入数组
int[] expectedNums = [...]; // 长度正确的期望答案

int k = removeDuplicates(nums); // 调用

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
如果所有断言都通过，那么您的题解将被 通过。



示例 1：

输入：nums = [1,1,2]
输出：2, nums = [1,2,_]
解释：函数应该返回新的长度 2 ，并且原数组 nums 的前两个元素被修改为 1, 2 。不需要考虑数组中超出新长度后面的元素。
示例 2：

输入：nums = [0,0,1,1,1,2,2,3,3,4]
输出：5, nums = [0,1,2,3,4]
解释：函数应该返回新的长度 5 ， 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4 。不需要考虑数组中超出新长度后面的元素。

**/
struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut slow: usize = 0;
        let len = nums.len();
        let mut fast: usize = 1;
        while fast < len {
            if nums[fast] != nums[slow] {
                slow += 1;
                nums[slow] = nums[fast];
            }
            fast += 1;
        }

        return (slow + 1) as i32;
    }

    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        return nums.len() as i32;
    }

    pub fn remove_duplicates3(nums: &mut Vec<i32>) -> i32 {
        // 下一个写入位置
        let mut idx = 0;
        // 遍历数组
        for i in 0..nums.len() {
            // idx == 0 第一个元素直接写入
            // 对比当前值与上一个写入的元素是否相等，如果不相等，则写入当前元素
            if (idx == 0 || nums[i] != nums[idx - 1]) {
                nums[idx] = nums[i];
                idx += 1;
            }
        }
        return idx as i32;
    }
}


#[cfg(test)]
mod tests {
    use crate::remove_duplicates::Solution;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 1, 2, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    }

    #[test]
    fn test_remove_duplicates2() {
        let mut nums = vec![1, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 3);
    }

    #[test]
    fn test_remove_duplicates3() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    }

    #[test]
    fn test_remove_duplicates4() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates3(&mut nums), 5);
    }
}