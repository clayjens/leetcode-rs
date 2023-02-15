use leetcode_rs::Solution;

#[test]
fn does_contain_duplicates() {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(Solution::contains_duplicate(nums), true);

    let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    assert_eq!(Solution::contains_duplicate(nums), true);
}

#[test]
fn does_not_contain_duplicates() {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::contains_duplicate(nums), false)
}
