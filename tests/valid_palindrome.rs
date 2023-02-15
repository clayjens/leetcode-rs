use leetcode_rs::Solution;

#[test]
fn is_valid_palindrome() {
    let s = String::from("A man, a plan, a canal: Panama");
    assert!(Solution::valid_palindrome(s));

    let s = String::from("race a car");
    assert!(!Solution::valid_palindrome(s));

    let s = String::from("");
    assert!(Solution::valid_palindrome(s));
}
