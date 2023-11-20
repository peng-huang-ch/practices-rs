#[allow(unused_macros)]
macro_rules! overload {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right), /* stringify!() is a built-in macro that converts an expression
                                 * into a string literal */
            $left && $right
        )
    };

    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}", stringify!($left), stringify!($right), $left || $right)
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_overload() {
        overload!(2u32 + 1 == 2; and 2u32 + 3 == 5);
        overload!(true; or false);
    }
}
