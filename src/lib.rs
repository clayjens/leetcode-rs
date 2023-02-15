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
}
