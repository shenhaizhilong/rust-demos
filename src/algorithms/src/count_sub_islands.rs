
/**
https://leetcode.com/problems/count-sub-islands/

Count Sub Islands using Rust

You are given two m x n binary matrices grid1 and grid2 containing only 0's (representing water) and 1's (representing land). An island is a group of 1's connected 4-directionally (horizontal or vertical). Any cells outside of the grid are considered water cells.

An island in grid2 is considered a sub-island if there is an island in grid1 that contains all the cells that make up this island in grid2.

Return the number of islands in grid2 that are considered sub-islands.

Example 1:


Input: grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]],
grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]
Output: 3
Explanation: In the picture above, the grid on the left is grid1 and the grid
on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island.
There are three sub-islands.
Example 2:


Input: grid1 = [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]],
grid2 = [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]
Output: 2
Explanation: In the picture above, the grid on the left is grid1 and the grid
on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island.
There are two sub-islands.
**/
pub fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for i in 0..grid2.len() {
        for j in 0..grid2[i].len() {
            if grid2[i][j] == 1 {
                if dfs2(&grid1, &mut grid2, i, j) {
                    ans += 1;
                }
            }
        }
    }

    return ans;
}

pub fn dfs2(grid1: &Vec<Vec<i32>>, grid2: &mut Vec<Vec<i32>>, r: usize, c: usize) -> bool {
    if grid2[r][c] == 0 {
        return true;
    }
    grid2[r][c] = 0;

    let mut is_sub_island = grid1[r][c] == 1;

    // 上、下、右、左
    let dirs = vec![vec![-1, 0], vec![1, 0], vec![0, 1], vec![0, -1]];
    for dir in dirs.iter() {
        // println!("{:?}", dir);
        let row = r as i32 + dir[0];
        let col = c as i32 + dir[1];
        if row >= 0 && col >= 0 && row < grid2.len() as i32 && col < grid2[0].len() as i32 && grid2[row as usize][col as usize] == 1 {
            is_sub_island = is_sub_island & dfs2(grid1, grid2, row as usize, col as usize);
            // 不能改成 is_sub_island = is_sub_island && dfs2(grid1, grid2, row as usize, col as usize); 因为 && 会短路后续的递归
        }
    }

    return is_sub_island;
}

#[cfg(test)]
mod tests {
    use crate::count_sub_islands::count_sub_islands;

    #[test]
    fn test_count_sub_islands() {
        assert_eq!(2, count_sub_islands(
            vec![
                vec![1, 0, 1, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 1, 0, 1]],
            vec![
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 0, 1, 0],
                vec![1, 0, 0, 0, 1]]));
    }
}