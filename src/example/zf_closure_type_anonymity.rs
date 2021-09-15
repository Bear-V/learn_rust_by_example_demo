/**
 * @Author: ZZX
 * @Description: 类型匿名 输入函数 输出参数
 * @Date: create in 2021/9/15 2:31 下午
 */

#[test]
fn one() {
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    let x = 7;

    let print = || println!("{}", x);

    apply(print)
}

#[test]
fn two() {
    fn call_me<F: Fn()>(f: F) {
        f()
    }

    fn function() {
        println!("i am a function");
    }

    let closure = || println!("i am a closure");

    call_me(function);
    call_me(closure);
}

#[test]
fn third() {
    fn create_fn() -> impl Fn() {
        let txt = "Fn".to_owned();

        move || println!("this is a {}", txt)
    }

    fn create_fn_mut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fn_once() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fn_mut();
    let fn_once = create_fn_once();

    fn_plain();
    fn_mut();
    fn_once();
}
