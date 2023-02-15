use std::collections::HashMap;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    /// #217. Contains Duplicate
    ///
    /// Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
    /// https://leetcode.com/problems/contains-duplicate/
    /// https://leetcode.com/problems/contains-duplicate/submissions/898708987/
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen_map: HashMap<i32, ()> = HashMap::new();

        for n in nums.iter() {
            if seen_map.insert(*n, ()).is_some() {
                return true;
            }
        }

        false
    }

    /// #242. Valid Anagram
    ///
    /// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
    /// https://leetcode.com/problems/valid-anagram/
    /// https://leetcode.com/problems/valid-anagram/submissions/898725382/
    pub fn valid_anagram(s: &str, t: &str) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_map: HashMap<char, i32> = HashMap::new();
        let mut t_map: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            *s_map.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            *t_map.entry(c).or_insert(0) += 1;
        }

        s_map == t_map
    }
}
