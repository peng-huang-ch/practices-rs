#[macro_export]
macro_rules! add {
    ($a: expr, $b: expr) => {{
        $a + $b
    }};
    ($a: expr) => {{
        $a + 1
    }};
}

#[macro_export]
macro_rules! add_as {
    ($a: expr, $b: expr, $typ:ty) => {{
        $a as $typ + $b as $typ
    }};
    ($($a: expr), *) => {{
        // to handle the case without any arguments
        0
        // block to be repeated
        $(+$a)*
    }};
    ($($a: expr), *) => {{
        let mut sum = 0;
        $({
            sum += $a;
        })*
        sum
    }};
    ($a:expr, $($b:tt)*) => {{
        $a + add_as!($($b)*)
    }};
}

#[macro_export]
macro_rules! vec {
    ($($a: expr), *) => {{
        let mut tmp = Vec::new();
        // block to be repeated
        $(
            tmp.push($a);
        )*
        tmp
    }};
}

#[macro_export]
macro_rules! ok_or_return {
    (@error $a: ident($($b:tt)*)) => {
        match $a($($b)*) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        }
    };
    ($a:ident($($b:tt)*)) => {
        ok_or_return!(@error $a($($b)*))
    };
}

#[allow(dead_code)]
fn some_work(i: i64, j: i64) -> Result<(i64, i64), String> {
    if i + j > 2 {
        Ok((i, j))
    } else {
        Err("error".to_owned())
    }
}

#[test]
fn test_marco() -> Result<(), String> {
    // ok_or_return! {some_work(1,4)};
    // ok_or_return! {some_work(1,0)};
    ok_or_return!(some_work(1, 4));
    // ok_or_return!(some_work(1, 0));
    Ok(())
}
