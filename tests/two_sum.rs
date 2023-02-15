use leetcode_rs::Solution;

#[test]
fn two_sum() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);

    // ---

    let nums = vec![3, 2, 4];
    let target = 6;

    assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);

    // ---

    let nums = vec![3, 3];
    let target = 6;

    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
}
