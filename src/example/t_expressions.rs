/**
 * @Author: ZZX
 * @Description: 表达式
 * @Date: create in 2021/9/14 10:29 上午
 */
#[test]
fn one() {
    // 变量绑定
    //     let x = 5;
    // 表达式
    // x;
    // x + 1;
    // 15;
    // 代码块也是表达式，所以它们在赋值操作中可以充当右值（r-values）。
    // 代码块中的最后一条 表达式将赋给左值（l-value）。
    // 需要注意的是，如果代码块最后一条表达式结尾处有分号，那 么返回值将变成 ()。
    // （译注：代码块中的最后一条语句是代码块中实际执行的最后一条语句，而不一 定是代码块中最后一行的语句。）

    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        // 将此表达式赋给 `y`
        x_cube + x_squared
    };

    #[allow(unused_must_use)]
    let z = {
        // 分号结束了这个表达式，于是将 `()` 赋给 `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

}
