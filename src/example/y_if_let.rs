/**
 * @Author: ZZX
 * @Description: if let
 * @Date: create in 2021/9/14 4:39 下午
 */

#[test]
fn one() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("this is a really long string and `{:?}`", i);
        }
        _ => {}
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    let a = if let Some(i) = number {
        println!("Matched {:?}", i);
        i
    } else {
        println!("asd");
        0
    };
    println!("{}", a);

    if let Some(i) = letter {
        println!("matched :{:?}", i);
    } else {
        println!("Didn't match a number. let's go with a letter");
    }

    // 提供另一种失败情况下的条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件的值为 false。于是以下是默认的分支：
        println!("I don't like letters. Let's go with an emoticon :)!");
    };

    // 以这个 enum 类型为例
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    // 创建变量
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // 变量 a 匹配到了 Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // 变量 c 匹配到了 Foo::Qux，它带有一个值，就和上面例子中的 Some() 类似。
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}
