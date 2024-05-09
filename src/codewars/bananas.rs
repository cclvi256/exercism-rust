fn bananas(s: &str) -> Vec<String> {
    todo!()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::bananas;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    
    fn do_test(s: &str, expected: Vec<&str>) {
        let actual:HashSet<String> = HashSet::from_iter(bananas(s));
        assert_eq!(actual, HashSet::from_iter(expected.iter().map(|x| x.to_string())), "s = \"{s}\"");
    }
    
    
    #[test]
    fn example_0() {
        do_test("banann", Vec::new());
    }
    #[test]
    fn example_1() {
        do_test("banana", vec!["banana"]);
    }
    #[test]
    fn example_2() {
        do_test("bbananana", vec!["b-an--ana", "-banana--", "-b--anana", "b-a--nana", "-banan--a",
                                  "b-ana--na", "b---anana", "-bana--na", "-ba--nana", "b-anan--a",  "-ban--ana", "b-anana--"]);
    }
    #[test]
    fn example_3() {
        do_test("bananaaa", vec!["banan-a-", "banana--", "banan--a"]);
    }
    #[test]
    fn example_4() {
        do_test("bananana", vec!["ban--ana", "ba--nana", "bana--na", "b--anana", "banana--", "banan--a"]);
    }
}