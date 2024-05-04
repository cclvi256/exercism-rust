pub fn is_armstrong_number(num: u32) -> bool {
    let separated_digits = digits(num);
    let digit_count = separated_digits.len();
    let mut sum: u64 = 0;
    for digit in separated_digits {
        sum += digit.pow(digit_count as u32) as u64;
    }
    sum == num as u64
}

fn digits (n: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    
    digits
}

// Sometimes it's necessary to use 64-bit integers.