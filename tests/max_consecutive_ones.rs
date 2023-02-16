use leetcode_rs::Solution;

#[test]
fn max_consecutive_ones() {
    let nums = vec![1, 1, 0, 1, 1, 1];
    assert_eq!(Solution::max_consecutive_ones(nums), 3);

    let nums = vec![1, 0, 1, 1, 0, 1];
    assert_eq!(Solution::max_consecutive_ones(nums), 2);
}
