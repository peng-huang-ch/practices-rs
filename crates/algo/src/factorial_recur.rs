#[allow(dead_code)]

fn factorial_recur(n: i32) -> i32 {
    if n == 0 {
        return 1
    }
    n * factorial_recur(n - 1)
}

#[test]
fn test_factorial_recur() {
    let res = factorial_recur(3);
    assert_eq!(res, 6);
}
