/**
 * @Author: ZZX
 * @Description: 泛型
 * @Date: create in 2021/9/16 2:40 下午
 */

// 机构体
#[test]
fn one() {
    struct A;

    struct Single(A);

    struct SingleGen<T>(T);

    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}

// 函数
#[test]
fn two() {
    struct A; // 具体类型 `A`。
    struct S(A); // 具体类型 `S`。
    struct SGen<T>(T); // 泛型类型 `SGen`。

    // 下面全部函数都得到了变量的所有权，并立即使之离开作用域，将变量释放。

    // 定义一个函数 `reg_fn`，接受一个 `S` 类型的参数 `_s`。
    // 因为没有 `<T>` 这样的泛型类型参数，所以这不是泛型函数。
    fn reg_fn(_s: S) {}

    // 定义一个函数 `gen_spec_t`，接受一个 `SGen<A>` 类型的参数 `_s`。
    // `SGen<>` 显式地接受了类型参数 `A`，且在 `gen_spec_t` 中，`A` 没有被用作
    // 泛型类型参数，所以函数不是泛型的。
    fn gen_spec_t(_s: SGen<A>) {}

    // 定义一个函数 `gen_spec_i32`，接受一个 `SGen<i32>` 类型的参数 `_s`。
    // `SGen<>` 显式地接受了类型参量 `i32`，而 `i32` 是一个具体类型。
    // 由于 `i32` 不是一个泛型类型，所以这个函数也不是泛型的。
    fn gen_spec_i32(_s: SGen<i32>) {}

    // 定义一个函数 `generic`，接受一个 `SGen<T>` 类型的参数 `_s`。
    // 因为 `SGen<T>` 之前有 `<T>`，所以这个函数是关于 `T` 的泛型函数。
    fn generic<T>(_s: SGen<T>) {}

    // 使用非泛型函数
    reg_fn(S(A)); // 具体类型。
    gen_spec_t(SGen(A)); // 隐式地指定类型参数 `A`。
    gen_spec_i32(SGen(6)); // 隐式地指定类型参数 `i32`。

    // 为 `generic()` 显式地指定类型参数 `char`。
    generic::<char>(SGen('a'));

    // 为 `generic()` 隐式地指定类型参数 `char`。
    generic(SGen('c'));
}

// 实现
#[test]
#[allow(dead_code)]
fn third() {
    struct S; // 具体类型 `S`
    struct GenericVal<T>(T); // 泛型类型 `GenericVal`

    // GenericVal 的 `impl`，此处我们显式地指定了类型参数：
    impl GenericVal<f32> {} // 指定 `f32` 类型
    impl GenericVal<S> {} // 指定为上面定义的 `S`

    // `<T>` 必须在类型之前写出来，以使类型 `T` 代表泛型。
    impl<T> GenericVal<T> {}

    struct Val {
        val: f64,
    }

    struct GenVal<T> {
        gen_val: T,
    }

    // Val 的 `impl`
    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    // GenVal 的 `impl`，指定 `T` 是泛型类型
    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}

// trait
#[test]
fn fourth() {
    // 不可复制的类型。
    struct Empty;
    struct Null;

    // `T` 的泛型 trait。
    trait DoubleDrop<T> {
        // 定义一个调用者的方法，接受一个额外的参数 `T`，但不对它做任何事。
        fn double_drop(self, _: T);
    }

    // 对泛型的调用者类型 `U` 和任何泛型类型 `T` 实现 `DoubleDrop<T>` 。
    impl<T, U> DoubleDrop<T> for U {
        // 此方法获得两个传入参数的所有权，并释放它们。
        fn double_drop(self, _: T) {}
    }

    let empty = Empty;
    let null = Null;

    // 释放 `empty` 和 `null`。
    empty.double_drop(null);

    //empty;
    //null;
    // ^ 试一试：去掉这两行的注释。
}
