# 原理

对 Rust 来说普通函数就是一段代码。而 closure 和 C++ 类似：每个 closure 会创建一个匿名的`struct`，编译器会在当前上下文 Context 捕获 closure 代码中的外部变量然后塞进这个结构体`struct`里面

## 类型

- 所有函数都至少能调用一次，所以全都会实现 FnOnce。
  - 另外，对于那些不会移走匿名结构体中变量的 closure 实现 FnMut。
    - 并且，对于那些不会修改匿名结构体中变量的 closure 实现 Fn。

三者是包含的关系

![https://ioover.net/media/Rust%20-%20Closure.png](https://ioover.net/media/Rust%20-%20Closure.png)

> 其中 FnMut 和 Fn 能调用多次。 FnMut 调用时需要对自己匿名结构体的 &mut self 引用。调用 Fn 只需要 &self 引用就足够了。

其中:

1. FnOnce, 参数类型是 self, 所以需要获取 f 的所有权，生命周期是短暂的
2. FnMut, Fn 的参数都是 self 的引用，所以可以调用多次

## `FnOnce`

标准库定义

```rs
#[lang = "fn_once"]
pub trait FnOnce<Args> {
    type Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
```

> 参数类型是 self，所以，这种类型的闭包会获取变量的所有权，生命周期只能是当前作用域，之后就会被释放了

## `FnMut`

标准库定义

```rs
#[lang = "fn_mut"]
pub trait FnMut<Args>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}
```

> 参数类型是&mut self，所以，这种类型的闭包是可变借用，会改变变量，但不会释放该变量。所以可以运行多次。

## `Fn`

标准库定义

```rs
#[lang = "fn"]
pub trait Fn<Args>: FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}
```

> 参数类型是&self，所以，这种类型的闭包是不可变借用，不会改变变量，也不会释放该变量。所以可以运行多次。
