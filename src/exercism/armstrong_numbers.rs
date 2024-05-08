pub fn is_armstrong_number(num: u32) -> bool {
    let separated_digits = digits(num);
    let digit_count = separated_digits.len();
    let mut sum: u64 = 0;
    for digit in separated_digits {
        sum += digit.pow(digit_count as u32) as u64;
    }
    sum == num as u64
}

fn digits(n: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    
    digits
}

// Sometimes it's necessary to use 64-bit integers.

#[test]
fn zero_is_an_armstrong_number() {
    assert!(is_armstrong_number(0))
}

#[test]
fn single_digit_numbers_are_armstrong_numbers() {
    assert!(is_armstrong_number(5))
}

#[test]
fn there_are_no_2_digit_armstrong_numbers() {
    assert!(!is_armstrong_number(10))
}

#[test]
fn three_digit_armstrong_number() {
    assert!(is_armstrong_number(153))
}

#[test]
fn three_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(100))
}

#[test]
fn four_digit_armstrong_number() {
    assert!(is_armstrong_number(9474))
}

#[test]
fn four_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(9475))
}

#[test]
fn seven_digit_armstrong_number() {
    assert!(is_armstrong_number(9_926_315))
}

#[test]
fn seven_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(9_926_316))
}

#[test]
fn nine_digit_armstrong_number() {
    assert!(is_armstrong_number(912_985_153));
}

#[test]
fn nine_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(999_999_999));
}

#[test]
fn ten_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(3_999_999_999));
}

// The following number has an Armstrong sum equal to 2^32 plus itself,
// and therefore will be detected as an Armstrong number if you are
// incorrectly using wrapping arithmetic.
#[test]
fn properly_handles_overflow() {
    assert!(!is_armstrong_number(4_106_098_957));
}

