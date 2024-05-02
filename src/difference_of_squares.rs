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
