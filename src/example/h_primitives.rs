/**
 * @Author: ZZX
 * @Description: 原生类型
 * @Date: create in 2021/9/6 22:55
 */
#[test]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn one() {
    // 标准类型 scalar type
    // 有符号整型 signed integers
    // i8, i16, i32 , i64 , isize
    // 无符号整型 unsigned integers
    // u8 , u16, u32 , u64 , usize
    // 浮点类型 floating point
    // f32 , f64
    // char 字符
    // 'a' 每个都是4字节
    // bool 布尔
    // true ， false
    // 单元类型
    // （） 唯一值 空元组
    // 复合类型 compound type
    // 数组 array
    // [ 1 ,2 ,3]
    // 元组 tuple
    // （ 1， true）

    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    // 报错！变量的类型并不能改变。
    // _mutable = true;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let mutable = true;
}
