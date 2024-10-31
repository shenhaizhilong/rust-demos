use std::collections::HashSet;


/**

https://leetcode.com/problems/count-the-number-of-consistent-strings/submissions/1438017315/

https://medium.com/@everythingismindgame/rust-cpp-python-java-compared-count-the-number-of-consistent-strings-74eed4f4467c

You are given a string allowed consisting of distinct characters and an array of strings words. A string is consistent if all characters in the string appear in the string allowed.

Return the number of consistent strings in the array words.

Example 1:

Input: allowed = "ab", words = ["ad","bd","aaab","baa","badab"]
Output: 2
Explanation: Strings "aaab" and "baa" are consistent since they only contain characters 'a' and 'b'.
Example 2:

Input: allowed = "abc", words = ["a","b","c","ab","ac","bc","abc"]
Output: 7
Explanation: All strings are consistent.
Example 3:

Input: allowed = "cad", words = ["cc","acd","b","ba","bac","bad","ac","d"]
Output: 4
Explanation: Strings "cc", "acd", "ac", and "d" are consistent.
Approach. O(mn)
Logic
Insert all characters of string allowed into unordered_set
count the number of output strings into int count.
Iterate over all word from vector<words>.
Iterate over all characters of word
if any character is not found in allowed unordered_set. Donot increment the count.
Complexity
Time: O(mn). m=Number of strings in words. n=Length of max word
Space: O(x). x=Length of allowed string
Code
**/
pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let set: HashSet<char> = allowed.chars().collect();
    let mut count = words.len() as i32;
    words.iter().for_each(|word| {
        for c in word.chars() {
            if !set.contains(&c) {
                count -= 1;
                break;
            }
        }
    });

    count
}

pub fn count_consistent_strings2(allowed: String, words: Vec<String>) -> i32 {
    let mut hash = [false; 256];
    for c in allowed.chars() {
        hash[c as usize] = true;
    }
    words.into_iter().filter(|word| word.chars().all(|c| hash[c as usize])).count() as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_consistent_strings() {
        let words = vec!["a", "b", "c", "ab", "ac", "bc", "abc"].iter().map(|s| s.to_string()).collect();
        println!("{}", count_consistent_strings("abc".to_string(), words));
    }

    #[test]
    fn test_count_consistent_strings2() {
        let words = vec!["a", "b", "c", "ab", "ac", "bc", "abc"].iter().map(|s| s.to_string()).collect();
        println!("{}", count_consistent_strings2("abc".to_string(), words));
    }
}

