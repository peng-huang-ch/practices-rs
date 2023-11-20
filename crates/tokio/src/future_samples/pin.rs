// https://course.rs/advance/async/pin-unpin.html
#[cfg(test)]
mod test_pin {
    use futures::Future;

    #[tokio::main]
    #[test]
    async fn test_pin() {
        #[derive(Debug)]
        struct Test {
            a: String,
            b: *const String,
        }

        impl Test {
            fn new(txt: &str) -> Self {
                Test { a: String::from(txt), b: std::ptr::null() }
            }

            fn init(&mut self) {
                let self_ref: *const String = &self.a;
                self.b = self_ref;
            }

            fn a(&self) -> &str {
                &self.a
            }

            fn b(&self) -> &String {
                assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
                unsafe { &*(self.b) }
            }
        }

        let mut test1 = Test::new("test1");
        test1.init();

        let mut test2 = Test::new("test2");
        test2.init();

        println!("a: {}, b: {}", test1.a(), test1.b());

        std::mem::swap(&mut test1, &mut test2);
        println!("a: {}, b: {}", test2.a(), test2.b());
    }

    #[tokio::main]
    #[test]
    async fn test_changed() {
        use std::{marker::PhantomPinned, pin::Pin};

        #[derive(Debug)]
        struct Test {
            a: String,
            b: *const String,
            _marker: PhantomPinned,
        }

        impl Test {
            fn new(txt: &str) -> Pin<Box<Self>> {
                let t = Test {
                    a: String::from(txt),
                    b: std::ptr::null(),
                    _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
                };
                let mut boxed = Box::pin(t);
                let self_ptr: *const String = &boxed.as_ref().a;
                unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
                boxed
            }

            fn a(self: Pin<&Self>) -> &str {
                &self.get_ref().a
            }

            fn b(self: Pin<&Self>) -> &String {
                assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
                unsafe { &*(self.b) }
            }
        }

        let mut test1 = Test::new("test1");
        let mut test2 = Test::new("test2");

        println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
        std::mem::swap(&mut test1, &mut test2);
        println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
    }

    #[tokio::main]
    #[test]
    async fn test_async() {
        // 多个不同的 `async` 语句块可以访问同一个本地变量，只要它们在该变量的作用域内执行
        async fn blocks() {
            let my_string = "foo".to_string();

            let future_one = async {
                // ...
                println!("{my_string}");
            };

            let future_two = async {
                // ...
                println!("{my_string}");
            };

            // 运行两个 Future 直到完成
            let ((), ()) = futures::join!(future_one, future_two);
        }

        // 由于 `async move` 会捕获环境中的变量，因此只有一个 `async move` 语句块可以访问该变量，
        // 但是它也有非常明显的好处： 变量可以转移到返回的 Future 中，不再受借用生命周期的限制
        fn move_block() -> impl Future<Output = ()> {
            let my_string = "foo".to_string();
            async move {
                // ...
                println!("{my_string}");
            }
        }

        move_block().await;
        blocks().await;
    }
}
