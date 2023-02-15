use leetcode_rs::Solution;

#[test]
fn is_valid_anagram() {
    let s = "anagram";
    let t = "nagaram";

    assert_eq!(Solution::valid_anagram(s, t), true);
}

#[test]
fn is_not_valid_anagram() {
    let s = "rat";
    let t = "car";

    assert_eq!(Solution::valid_anagram(s, t), false);

    // ---

    let s = "aacc";
    let t = "ccac";

    assert_eq!(Solution::valid_anagram(s, t), false);
}
