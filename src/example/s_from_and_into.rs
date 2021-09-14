/**
* @Author: ZZX
* @Description: From 和 Into 两个 trait 是内部相关联的，
   实际上这是它们实现的一部分。
   如果我们能够从类型 B 得到类型 A，
   那么很容易相信我们也能把类型 B 转换为类型 A
* @Date: create in 2021/9/13 6:08 下午
*/

#[test]
fn one() {
    let my_str = "hello world";
    let _my_string = String::from(my_str);

    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30);
    println!("my number is {:?}", num);

    let int = 5;

    let num_new: Number = int.into();
    println!("my number other is {:?}", num_new);
}

/*
    TryFrom & TryInto
    类似于 From 和 Into，TryFrom 和 TryInto 是 类型转换的通用 trait。
    不同于 From/Into 的是，TryFrom 和 TryInto trait 用于易出错的转换
    也正因如此，其返回值是 Result 型
*/

#[test]
fn two() {
    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

/*
    解析字符串
    我们经常需要把字符串转成数字。
    完成这项工作的标准手段是用 parse 函数。
    我们得 提供要转换到的类型，这可以通过不使用类型推断，或者用 “涡轮鱼” 语法（turbo fish，<>）实现。

    只要对目标类型实现了 FromStr trait，
    就可以用 parse 把字符串转换成目标类型。
    标准库中已经给无数种类型实现了 FromStr。
    如果要转换到用户定义类型，只要手动实现 FromStr 就行。
 */

#[test]
fn third() {
    let parsed:i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum is {:?}",sum);
}