pub fn square_of_sum(n: u32) -> u32 {
    let sum = n * (n + 1) / 2;
    (sum * sum) as u32
}

pub fn sum_of_squares(n: u32) -> u32 {
    let sum = n * (n + 1) * (2 * n + 1) / 6;
    sum as u32
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

// It seems that this exercise is for loop, not using the formula. But using formula is more efficient.

#[test]
fn square_of_sum_1() {
    assert_eq!(1, square_of_sum(1));
}

#[test]
fn square_of_sum_5() {
    assert_eq!(225, square_of_sum(5));
}

#[test]
fn square_of_sum_100() {
    assert_eq!(25_502_500, square_of_sum(100));
}

#[test]
fn sum_of_squares_1() {
    assert_eq!(1, sum_of_squares(1));
}

#[test]
fn sum_of_squares_5() {
    assert_eq!(55, sum_of_squares(5));
}

#[test]
fn sum_of_squares_100() {
    assert_eq!(338_350, sum_of_squares(100));
}

#[test]
fn difference_1() {
    assert_eq!(0, difference(1));
}

#[test]
fn difference_5() {
    assert_eq!(170, difference(5));
}

#[test]
fn difference_100() {
    assert_eq!(25_164_150, difference(100));
}
