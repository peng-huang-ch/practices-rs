#[allow(dead_code)]
pub fn for_loop_recur(n: i32) -> i32 {
    let mut stack = Vec::new();
    let mut res = 0;
    for i in (1..=n).rev() {
        stack.push(i);
    }
    while !stack.is_empty() {
        res += stack.pop().unwrap();
    }
    res
}

#[test]
fn test_for_loop_recur() {
    let res = for_loop_recur(199_i32);
    assert_eq!(res, 19900);
}
