use super::*;

#[test]
fn empty_input_sums_to_0() {
    // given a next-digit-matching-sum function
    let sut = sum_matching_digits;

    // when the function is called with empty input
    let input = "";
    let result = sut(input);

    // then the result is 0
    let expected_result = Some(0);
    assert_eq!(result, expected_result);
}

#[test]
fn non_matching_input_sums_to_0() {
    // given a next-digit-matching-sum function
    let sut = sum_matching_digits;

    // when the function is called with non-matching input
    let input = "01234567890123456789";
    let result = sut(input);

    // then the result is 0
    let expected_result = Some(0);
    assert_eq!(result, expected_result);
}

#[test]
fn matching_1s_sum_to_2() {
    // given a next-digit-matching-sum function
    let sut = sum_matching_digits;

    // when the function is called with non-matching input
    let input = "111";
    let result = sut(input);

    // then the result is 3
    let expected_result = Some(3);
    assert_eq!(result, expected_result);
}

#[test]
fn invalid_non_matching_input_is_silently_rejected() {
    // given a next-digit-matching-sum function
    let sut = sum_matching_digits;

    // when the function is called with invalid non_matching input
    let input = "a1b1c1d";
    let result = sut(input);

    // then the result is 0
    let expected_result = Some(0);
    assert_eq!(result, expected_result);
}

#[test]
fn invalid_matching_input_is_silently_rejected() {
    // given a next-digit-matching-sum function
    let sut = sum_matching_digits;

    // when the function is called with invalid non_matching input
    let input = "a2b22c2d";
    let result = sut(input);

    // then the result is 2
    let expected_result = Some(2);
    assert_eq!(result, expected_result);
}

#[test]
fn overflowing_sum_returns_none() {
    // given a next-digit-matching-summation function
    let sut = sum_matching_digits_iter;

    // when the function is called with input exceeding u32::MAX
    let init_val = (Some(std::u32::MAX / 9 * 9), Some(9));
    let input = "9";
    let result = sut(init_val, input.chars());

    // then the result is none
    let expected_result = None;
    assert_eq!(result, expected_result);
}

#[test]
fn single_digit_does_not_match_itself() {
    // given a next-digit-matching-sum function
    let sut = sum_matching_digits;

    // when the function is called with non-matching input
    let input = "1";
    let result = sut(input);

    // then the result is 0
    let expected_result = Some(0);
    assert_eq!(result, expected_result);
}

#[test]
fn single_digit_with_invalid_digit_does_not_match_itself() {
    // given a next-digit-matching-sum function
    let sut = sum_matching_digits;

    // when the function is called with non-matching input
    let input = "1x";
    let result = sut(input);

    // then the result is 0
    let expected_result = Some(0);
    assert_eq!(result, expected_result);
}
