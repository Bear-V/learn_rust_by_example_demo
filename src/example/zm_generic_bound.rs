use std::fmt::Display;

/**
 * @Author: ZZX
 * @Description: 泛型 约束
 * @Date: create in 2021/9/16 2:49 下午
 */

#[test]
#[allow(dead_code)]
fn one() {
    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }
}

#[test]
fn two() {
    // struct S<T: Display>(T);

    // let _s = S(vec![1]);
}

#[test]
fn third() {
    use std::fmt::Debug;

    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.height
        }
    }
    #[derive(Debug)]
    struct Rectangle {
        length: f64,
        height: f64,
    }
    #[allow(dead_code)]
    struct Triangle {
        length: f64,
        height: f64,
    }

    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    fn area<T: HasArea>(t: &T) -> f64 {
        t.area()
    }

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);

    println!("Area:{}", area(&rectangle));
}

// 空约束
#[test]
fn fourth() {
    struct Cardinal;
    struct BlueJay;
    struct Turkey;

    trait Red {}
    trait Blue {}

    impl Red for Cardinal {}
    impl Blue for BlueJay {}

    // 这些函数只对实现了相应的 trait 的类型有效。
    // 事实上这些 trait 内部是空的，但这没有关系。
    fn red<T: Red>(_: &T) -> &'static str {
        "red"
    }
    fn blue<T: Blue>(_: &T) -> &'static str {
        "blue"
    }

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // 由于约束，`red()` 不能作用于 blue_jay （蓝松鸟），反过来也一样。
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ 试一试：去掉此行注释。
}

// 多重约束
#[test]
fn fifth() {
    use std::fmt::{Debug, Display};

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}", t);
        println!("u: `{:?}", u);
    }

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // 试一试 ^ 将此行注释去掉。

    compare_types(&array, &vec);
}
