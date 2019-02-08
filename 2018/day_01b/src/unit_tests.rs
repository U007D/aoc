use super::*;

#[test]
fn test_case_1() {
    assert_eq!(Device::first_duplicate_frequency(vec![1, -1]), Ok(0));
}

#[test]
fn test_case_2() {
    assert_eq!(Device::first_duplicate_frequency(vec![3, 3, 4, -2, -4]), Ok(10));
}

#[test]
fn test_case_3() {
    assert_eq!(Device::first_duplicate_frequency(vec![-6, 3, 8, 5, -6]), Ok(5));
}

#[test]
fn test_case_4() {
    assert_eq!(Device::first_duplicate_frequency(vec![7, 7, -2, -7, -4]), Ok(14));
}
