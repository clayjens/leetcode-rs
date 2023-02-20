#![allow(unused_variables)]

use std::collections::HashMap;
use std::cmp::Ordering;

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

    /// #53. Maximum Subarray
    /// Given an integer array nums, find the subarray with the largest sum, and return its sum.
    ///
    /// https://leetcode.com/problems/maximum-subarray/
    /// https://leetcode.com/problems/maximum-subarray/submissions/898836118/
    pub fn maximum_subarray(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut curr_sum = nums[0];

        for i in 1..nums.len() {
            curr_sum = std::cmp::max(nums[i], curr_sum + nums[i]);
            max_sum = std::cmp::max(max_sum, curr_sum);
        }

        max_sum
    }

    /// #125. Valid Palindrome
    /// Given a string s, return true if it is a palindrome, or false otherwise.
    ///
    /// https://leetcode.com/problems/valid-palindrome/
    /// https://leetcode.com/problems/valid-palindrome/submissions/898747870/
    pub fn valid_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().filter(|s| s.is_alphanumeric()).collect();

        if chars.len() == 0 {
            return true;
        }

        let mut left = 0;
        let mut right = chars.len() - 1;

        while left < right {
            let left_c = chars[left].to_ascii_lowercase();
            let right_c = chars[right].to_ascii_lowercase();

            if !left_c.is_alphanumeric() || !right_c.is_alphanumeric() {
                continue;
            }

            if left_c == right_c {
                left += 1;
                right -= 1;
            } else {
                return false;
            }
        }

        true
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

    /// #485. Max Consecutive Ones
    /// Given a binary array nums, return the maximum number of consecutive 1's in the array.
    ///
    /// https://leetcode.com/problems/max-consecutive-ones/
    ///
    pub fn max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut len = 0;
        let mut curr = 0;

        for n in nums {
            match n {
                1 => curr += 1,
                _ => curr = 0,
            }

            len = std::cmp::max(len, curr);
        }

        len
    }
    
    /// #35. Search Insert Position
    /// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
    /// 
    /// https://leetcode.com/problems/search-insert-position/
    ///
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            match target.cmp(&nums[mid]) {
                Ordering::Less => {
                    if mid <= 0 { break; }
                    right = mid - 1;
                },
                Ordering::Greater => left = mid + 1,
                Ordering::Equal => return mid as i32
            }
        }

        left as i32
    }
}
