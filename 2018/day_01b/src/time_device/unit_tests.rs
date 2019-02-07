use super::*;

fn values_to_results_of_string(values: Vec<i32>) -> Vec<Result<i32>> {
    values.into_iter()
          .map(Ok)
          .collect()
}

#[test]
fn test_case_1() {
    let deltas = values_to_results_of_string(vec![1, -1]);
    assert_eq!(TimeDevice::first_duplicate_frequency(deltas), Ok(0));
}

#[test]
fn test_case_2() {
    let deltas = values_to_results_of_string(vec![3, 3, 4, -2, -4]);
    assert_eq!(TimeDevice::first_duplicate_frequency(deltas), Ok(10));
}

#[test]
fn test_case_3() {
    let deltas = values_to_results_of_string(vec![-6, 3, 8, 5, -6]);
    assert_eq!(TimeDevice::first_duplicate_frequency(deltas), Ok(5));
}

#[test]
fn test_case_4() {
    let deltas = values_to_results_of_string(vec![7, 7, -2, -7, -4]);
    assert_eq!(TimeDevice::first_duplicate_frequency(deltas), Ok(14));
}
