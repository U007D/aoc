use super::*;

#[test]
fn test_case_1() {
    let ids = vec!["abcdef", "bababc", "abbcde", "abcccd",
                   "aabcdd", "abcdee", "ababab"].iter()
                                                .map(|s| s.to_string())
                                                .collect::<Vec<_>>();
    assert_eq!(Device::checksum(ids), 12);
}
