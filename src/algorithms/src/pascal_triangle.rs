/**
https://leetcode.com/problems/pascals-triangle-ii/description/

Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.

In Pascal’s triangle, each number is the sum of the two numbers directly above it as shown:


Example 1:

Input: rowIndex = 3
Output: [1,3,3,1]
Example 2:

Input: rowIndex = 0
Output: [1]
Example 3:

Input: rowIndex = 1
Output: [1,1]
Approach-1. O(n2)
Logic
1st and last elements of every row are always 1.
pascalArr[row][0]=1;
pascalArr[row][size]=1.

Other elements = `v[row][col] = v[row-1][col-1] + v[row-1][col]`
Create a pascal Array till rowIndex
    0  1  2  3  4
0   1
1   1  1
2   1  2  1
3   1  3  3  1
4   1  4  6  4  1
Return only asked array only taking rowIndex elements.
**/
pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    if row_index < 1 {
        result.push(1);
        return result;
    }

    if row_index == 1 {
        result.push(1);
        result.push(1);
        return result;
    }

    let mut curr_row: Vec<i32> = Vec::new();
    curr_row.push(1);
    curr_row.push(1);
    let n = row_index + 1;
    let mut r = 3;
    while r <= n {
        let mut next_row: Vec<i32> = Vec::new();
        // 起始填1
        next_row.push(1);
        let mut i = 0;
        while (i as i32) < r - 2 {
            next_row.push(curr_row[i] + curr_row[i + 1]);
            i += 1;
        }
        // 起始填1
        next_row.push(1);
        curr_row = next_row;
        r += 1;
    }


    return curr_row;
}

pub fn get_row2(row_index: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    result.push(1);
    if row_index < 1 {
        return result;
    }
    result.push(1);
    if row_index == 1 {
        return result;
    }
    while result.len() - 1 < row_index as usize {
        let mut next_row: Vec<i32> = Vec::new();
        next_row.push(1);
        for i in 0..result.len() - 1 {
            next_row.push(result[i] + result[i + 1]);
        }
        next_row.push(1);
        result = next_row;
    }


    return result;
}

#[cfg(test)]
mod tests {
    use crate::pascal_triangle::{get_row, get_row2};

    #[test]
    fn test_get_row() {
        assert_eq!(vec![1], get_row(0));
        assert_eq!(vec![1, 1], get_row(1));
        assert_eq!(vec![1, 2, 1], get_row(2));
        assert_eq!(vec![1, 3, 3, 1], get_row(3));
        assert_eq!(vec![1, 4, 6, 4, 1], get_row(4));
        assert_eq!(vec![1, 5, 10, 10, 5, 1], get_row(5));
    }

    #[test]
    fn test_get_row2() {
        assert_eq!(vec![1], get_row2(0));
        assert_eq!(vec![1, 1], get_row2(1));
        assert_eq!(vec![1, 2, 1], get_row2(2));
        assert_eq!(vec![1, 3, 3, 1], get_row2(3));
        assert_eq!(vec![1, 4, 6, 4, 1], get_row2(4));
        assert_eq!(vec![1, 5, 10, 10, 5, 1], get_row2(5));
    }
}