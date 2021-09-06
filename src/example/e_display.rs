/**
 * @Author: ZZX
 * @Description: 显示
 * @Date: create in 2021/9/6 22:55
 */
use std::fmt;
use std::fmt::Formatter;

#[test]
fn one() {
    #[derive(Debug)]
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    let s = Structure(32);
    println!("{}", s);
}

#[test]
fn two() {
    #[derive(Debug)]
    struct MinMax(i64, i64);

    // 实现 MinMax 的 Display
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            // 使用 self.number 来表示各个数据
            write!(f, "{} ,{}", self.0, self.1)
        }
    }
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "x:{},y:{}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
    // 得到实现。这语句不能运行。
    // println!("What does Point2D look like in binary: {:b}?", point);
}

#[test]
fn practice() {
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
