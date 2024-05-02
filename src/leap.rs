pub fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[test]
fn is_leap_year_test() {
    // Test common non-leap years
    assert_eq!(is_leap_year(2014), false);
    assert_eq!(is_leap_year(2015), false);
    
    // Test common leap years
    assert_eq!(is_leap_year(2016), true);
    assert_eq!(is_leap_year(2020), true);
    
    // Test 100 divided but not 400 divided
    assert_eq!(is_leap_year(1900), false);
    assert_eq!(is_leap_year(2100), false);
    
    // Test 400 divided
    assert_eq!(is_leap_year(2000), true);
    assert_eq!(is_leap_year(2400), true);
}