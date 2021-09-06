/**
 * @Author: ZZX
 * @Description: 测试实例 List
 * @Date: create in 2021/9/6 22:55
 */
use std::fmt;
use std::fmt::Formatter;

#[test]
fn one() {
    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;

            for (index, v) in vec.iter().enumerate() {
                if index != 0 {
                    write!(f, ",")?;
                }
                write!(f, "{}", v)?;
            }

            write!(f, "]")
        }
    }
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

#[test]
fn practice() {
    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;

            for (index, value) in vec.iter().enumerate() {
                if index != 0 {
                    write!(f, ",")?
                };
                write!(f, "{}:{}", index, value)?;
            }

            write!(f, "]")
        }
    }

    let list = List(vec![1, 2, 3]);
    println!("{}", list);
}
