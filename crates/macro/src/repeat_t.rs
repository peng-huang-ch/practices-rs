#[allow(unused_macros)]
macro_rules! find_min {
    // Base case:
    ($x:expr) => {
        $x
    };
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => {
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_overload() {
        find_min!(1);
        let _ = find_min!(1u32 + 2, 2u32);
    }
}
