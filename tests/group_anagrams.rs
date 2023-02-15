use leetcode_rs::Solution;

#[test]
#[ignore = "unimplemented"]
fn group_anagrams() {
    let strs: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|s| String::from(*s))
        .collect();

    assert_eq!(
        Solution::group_anagrams(strs),
        vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
    );
}
