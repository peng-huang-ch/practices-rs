#[allow(unused_macros)]
macro_rules! create_function {
    ($fn_name:ident) => {
        fn $fn_name() {
            println!("You called {:?}()", stringify!($fn_name));
        }
    };
}

#[allow(unused_macros)]
macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_function() {
        create_function!(foo);
        create_function!(bar);
        foo();
        bar();
    }

    #[test]
    fn test_print_result() {
        print_result!(2u32 + 1);
        print_result!(2u32 + 3);
    }
}
