#![allow(unused_variables)]

use std::collections::HashMap;

#[derive(Debug)]
pub struct Solution;

impl Solution {
    /// #1. Two Sum
    /// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
    ///
    /// https://leetcode.com/problems/two-sum/
    /// https://leetcode.com/problems/two-sum/submissions/898734739/
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut compliment_map: HashMap<i32, usize> = HashMap::new();

        for n in nums.iter().enumerate() {
            match compliment_map.get(&n.1) {
                Some(i) => return vec![*i as i32, n.0 as i32],
                None => compliment_map.insert(target - n.1, n.0),
            };
        }

        vec![]
    }

    /// #49. Group Anagrams
    /// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
    ///
    /// https://leetcode.com/problems/group-anagrams/
    ///
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // TODO: https://github.com/neetcode-gh/leetcode/blob/main/rust/0049-group-anagrams.rs
        unimplemented!()
    }

    /// #125. Valid Palindrome
    /// Given a string s, return true if it is a palindrome, or false otherwise.
    ///
    /// https://leetcode.com/problems/valid-palindrome/
    /// https://leetcode.com/problems/valid-palindrome/submissions/898743336/
    pub fn valid_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .chars()
            .filter_map(|s| {
                if s.is_alphanumeric() {
                    return Some(s.to_ascii_lowercase());
                }
                None
            })
            .collect();

        let mut s_rev = s.clone();
        s_rev.reverse();

        s_rev == s
    }

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
