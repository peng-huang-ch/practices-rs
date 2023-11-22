#[allow(dead_code)]

fn exp_recur(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    exp_recur(n - 1) + exp_recur(n - 1) + 1
}

#[test]
fn test_exp_recur() {
    let res = exp_recur(10_i32);
    assert_eq!(res, 1023_i32);
}
