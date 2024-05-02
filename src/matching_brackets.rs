pub fn brackets_are_balanced(string: &str) -> bool {
    
    let mut stack = Vec::new();
    for bracket in string.chars() {
        match bracket {
            '(' | '[' | '{' => stack.push(bracket),
            ')' | ']' | '}' => {
                if let Some(last) = stack.pop() {
                    if !is_matching_pair(last, bracket) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => continue,
        }
    }
    stack.is_empty()
}

fn is_matching_pair(open: char, close: char) -> bool {
    matches!((open, close), ('(', ')') | ('[', ']') | ('{', '}'))
}

/*
Test strategy:
1 Test empty string
2 Test string with no brackets
3 Test string with only one type of bracket
    31 Balanced
    32 Unbalanced - too many left brackets
    33 Unbalanced - too many right brackets
4 Test string with multiple types of brackets
    41 Balanced
    42 Unbalanced
        421 Too many left brackets
        422 Too many right brackets
    43 Balanced with nested brackets
    44 Count matches, but not balanced
 */

#[test]
fn test1() {
    assert_eq!(brackets_are_balanced(""), true);
}

#[test] fn test2() {
    assert_eq!(brackets_are_balanced("Hello, World!"), true);
}

#[test] fn test31() {
    assert_eq!(brackets_are_balanced("()"), true);
}

#[test] fn test32() {
    assert_eq!(brackets_are_balanced("("), false);
}

#[test] fn test33() {
    assert_eq!(brackets_are_balanced(")"), false);
}

#[test] fn test41() {
    assert_eq!(brackets_are_balanced("()[]{}"), true);
}

#[test] fn test421() {
    assert_eq!(brackets_are_balanced("([{})"), false);
}

#[test] fn test422() {
    assert_eq!(brackets_are_balanced("([{}])}"), false);
}

#[test] fn test43() {
    assert_eq!(brackets_are_balanced("([{}])"), true);
}

#[test] fn test44() {
    assert_eq!(brackets_are_balanced("([)]{()[}]"), false);
}