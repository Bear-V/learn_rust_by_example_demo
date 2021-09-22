/**
 * @Author: ZZX
 * @Description: 虚拟型参数
 * @Date: create in 2021/9/16 3:48 下午
 */

#[test]
fn one() {
    use std::marker::PhantomData;
    // 这个虚元组结构体对 `A` 是泛型的，并且带有隐藏参数 `B`。
    #[derive(PartialEq)] // 允许这种类型进行相等测试（equality test）
    struct PhantomTuple<A, B>(A, PhantomData<B>);

    // 这个虚类型结构体对 `A` 是泛型的，并且带有隐藏参数 `B`。
    #[derive(PartialEq)] // 允许这种类型进行相等测试。
    struct PhantomStruct<A, B> {
        first: A,
        phantom: PhantomData<B>,
    }

    // 这里的 `f32` 和 `f64` 是隐藏参数。
    // 被指定为 `<char, f32>` 的 `PhantomTuple` 类型。
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // 被指定为 `<char, f64>` `PhantomTuple` 类型。
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // 被指定为 `<char, f32>` 的类型。
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    // 被指定为 `<char, f64>` 的类型。
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
}

#[test]
fn third() {
    use std::marker::PhantomData;
    use std::ops::Add;

    // 创建空枚举来表示单位
    #[derive(Debug, Clone, Copy)]
    enum Inch {}
    #[derive(Debug, Clone, Copy)]
    enum Mm {}

    /// `Length` 是一个带有虚类型参数 `Unit` 的类型，
    /// 而且对于表示长度的类型（即 `f64`）而言，`Length` 不是泛型的。
    ///
    /// `f64` 已经实现了 `Clone` 和 `Copy` trait
    #[derive(Debug, Clone, Copy)]
    struct Length<Unit>(f64, PhantomData<Unit>);
    impl<Unit> Add for Length<Unit> {
        type Output = Length<Unit>;

        fn add(self, rhs: Length<Unit>) -> Length<Unit> {
            Length(self.0 + rhs.0, PhantomData)
        }
    }
    // 指定 `one_foot` 拥有虚类型参数 `Inch`。
    let one_foot:  Length<Inch> = Length(12.0, PhantomData);
    // `one_meter` 拥有虚类型参数 `Mm`。
    let one_meter: Length<Mm>   = Length(1000.0, PhantomData);

    // `+` 调用了我们对 `Length<Unit>` 实现的 `add()` 方法。
    //
    // 由于 `Length` 了实现了 `Copy`，`add()` 不会消耗 `one_foot`
    // 和 `one_meter`，而是复制它们作为 `self` 和 `rhs`。
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // 加法正常执行。
    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);
}
