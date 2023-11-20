#![allow(dead_code)]

#[derive(Debug)]
struct E {
    a: String,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroyed struct E");
    }
}

fn fn_once<F>(func: F)
where
    F: FnOnce(),
{
    println!("fn_once begins");
    // func();
    func();
    println!("fn_once ended");
}

fn fn_immut<F>(f: F)
where
    F: Fn() -> String,
{
    println!("calling Fn closure from fn, {}", f());
}

fn fn_mut<F>(mut func: F)
where
    F: FnMut(),
{
    println!("fn_mut begins");
    func();
    func();
    println!("fn_mut ended");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_fn_once() {
        let e = E { a: "fn_once".to_string() };
        // 这样加个move，看看程序执行输出顺序有什么不同
        // let f = || println!("fn once calls: {:?}", e);
        let f = move || println!("fn once closure calls: {:?}", e);
        fn_once(f);
        println!("main ended");
    }

    #[test]
    fn test_fn_mut() {
        let mut e = E { a: "fn_mut".to_string() };
        // 这样加个move，看看程序执行输出顺序有什么不同
        // let f = || println!("fn once calls: {:?}", e);
        let f = move || {
            println!("FnMut closure calls: {:?}", e);
            e.a = "fn_mut".to_string();
        };
        // fn_once(f);
        fn_mut(f);
        println!("main ended");
    }
    #[test]
    fn test_fn_closure() {
        fn make_adder<'a>(x: i32) -> Box<dyn Fn(i32) -> i32> {
            Box::new(move |y| x + y)
        }

        let f = make_adder(2);
        println!("{}", f(1)); // 4
        println!("{}", f(10)); // 13
    }
}
