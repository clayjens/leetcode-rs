use leetcode_rs::Solution;

#[test]
fn maximum_subarray() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(Solution::maximum_subarray(nums), 6);

    // ---

    let nums = vec![1];
    assert_eq!(Solution::maximum_subarray(nums), 1);

    // ---

    let nums = vec![5, 4, -1, 7, 8];
    assert_eq!(Solution::maximum_subarray(nums), 23);

    // ---

    let nums = vec![-1, -2];
    assert_eq!(Solution::maximum_subarray(nums), -1);
}
