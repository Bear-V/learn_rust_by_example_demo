/**
* @Author: ZZX
* @Description: 闭包作为输入参数

   虽然 Rust 无需类型说明就能在大多数时候完成变量捕获，但在编写函数时，这种模糊写法 是不允许的。
   当以闭包作为输入参数时，必须指出闭包的完整类型，它是通过使用以下 trait 中的一种来指定的。
   其受限制程度按以下顺序递减：

       Fn：表示捕获方式为通过引用（&T）的闭包
       FnMut：表示捕获方式为通过可变引用（&mut T）的闭包
       FnOnce：表示捕获方式为通过值（T）的闭包
   ！ 顺序之所以是这样，是因为
       &T 只是获取了不可变的引用，
       &mut T 则可以改变 变量，
       T 则是拿到了变量的所有权而非借用。
* @Date: create in 2021/9/15 2:10 下午
*/

#[test]
fn one() {
    // 该函数将闭包作为参数并调用它。
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    // 输入闭包，返回一个 `i32` 整型的函数。
    fn apply_to_3<F>(f: F) -> i32
    where
        // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    use std::mem;

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("i said {}", greeting);

        farewell.push_str("!!!");
        println!("then i screamed {}", farewell);

        mem::drop(farewell)
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 double {}", apply_to_3(double));
}
