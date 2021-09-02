#[allow(dead_code)]
struct UnPrintable(i32);

// derive 属性会自动创建所需要的实现，是这个 结构体 可以被 fmt::Debug 打印
#[derive(Debug)]
#[allow(dead_code)]
struct DebugPrintable(i32);


#[test]
fn one() {
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    // 使用 `{:?}` 打印和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor = "actor's");

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));

    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}
