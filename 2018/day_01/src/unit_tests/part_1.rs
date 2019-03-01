use super::*;

#[test]
fn test_1() {
    assert_eq!(Device::calc_final_frequency(vec![1, 1, 1].iter()), Ok(3));
}

#[test]
fn test_2() {
    assert_eq!(Device::calc_final_frequency(vec![1, 1, -2].iter()), Ok(0));
}

#[test]
fn test_3() {
    assert_eq!(Device::calc_final_frequency(vec![-1, -2, -3].iter()), Ok(-6));
}
