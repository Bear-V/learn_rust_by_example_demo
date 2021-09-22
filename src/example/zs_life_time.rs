use std::fmt::Debug;

/**
 * @Author: ZZX
 * @Description: 生命周期
* @Date: create in 2021/9/18 5:45 下午
 */

// 生命周期
// 生命周期（lifetime）是这样一种概念
// ，编译器（中的借用检查器）用它来保证所有的 借用都是有效的。
// 确切地说，一个变量的生命周期在它创建的时候开始，在它销毁的时候 结束。
// 虽然生命周期和作用域经常被一起提到，但它们并不相同。
//
// 例如考虑这种情况，我们通过 & 来借用一个变量。
// 该借用拥有一个生命周期，此生命 周期由它声明的位置决定。
// 于是，只要该借用在出借者（lender）被销毁前结束，借用 就是有效的。
// 然而，借用的作用域则是由使用引用的位置决定的。
//
// 在下面的例子和本章节剩下的内容里，我们将看到生命周期和作用域的联系与区别。

#[test]
fn one() {
    // 下面使用连线来标注各个变量的创建和销毁，从而显示出生命周期。
    // `i` 的生命周期最长，因为它的作用域完全覆盖了 `borrow1` 和
    // `borrow2` 的。`borrow1` 和 `borrow2` 的周期没有关联，
    // 因为它们各不相交。
    {
        let i = 3; // Lifetime for `i` starts. ─────────────────┐
        {
            //                                                       │
            let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
            println!("borrow1: {}", borrow1); //                    ││
        } // `borrow1 ends. ────────────────────────────────────────┘│
          //                                                         │
          //                                                         │
        {
            //                                                       │
            let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
            println!("borrow2: {}", borrow2); //                    ││
        } // `borrow2` ends. ───────────────────────────────────────┘│
          //                                                         │
    } // Lifetime ends. ─────────────────────────────────────────────┘
}

/// 显式标注
///
/// 借用检查器使用显式的生命周期标记来明确引用的有效时间应该持续多久。
/// 在生命周期没有 省略1的情况下，Rust 需要显式标注来确定引用的生命周期应该是什么样的。
/// 可以用撇号 显式地标出生命周期，语法如下：
/// foo<'a>
/// `foo` 带有一个生命周期参数 `'a`
/// foo<'a ,'b >

#[test]
fn two() {
    // `print_refs` 接受两个 `i32` 的引用，它们有不同的生命周期 `'a` 和 `'b`。
    // 这两个生命周期都必须至少要和 `print_refs` 函数一样长。
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {} and y is {}", x, y);
    }

    // 不带参数的函数，不过有一个生命周期参数 `'a`。
    fn failed_borrow<'a>() {
        let _x = 12;

        // 报错：`_x` 的生命周期不够长
        //let y: &'a i32 = &_x;
        // 在函数内部使用生命周期 `'a` 作为显式类型标注将导致失败，因为 `&_x` 的
        // 生命周期比 `y` 的短。短生命周期不能强制转换成长生命周期。
    }
    {
        // 创建变量，稍后用于借用。
        let (four, nine) = (4, 9);

        // 两个变量的借用（`&`）都传进函数。
        print_refs(&four, &nine);
        // 任何被借用的输入量都必须比借用者生存得更长。
        // 也就是说，`four` 和 `nine` 的生命周期都必须比 `print_refs` 的长。

        failed_borrow();
        // `failed_borrow` 未包含引用，因此不要求 `'a` 长于函数的生命周期，
        // 但 `'a` 寿命确实更长。因为该生命周期从未被约束，所以默认为 `'static`。
    }
}

///函数
//
// 排除省略（elision）的情况，带上生命周期的函数签名有一些限制：
//
// 任何引用都必须拥有标注好的生命周期。
// 任何被返回的引用都必须有和某个输入量相同的生命周期或是静态类型（static）。
// 另外要注意，如果没有输入的函数返回引用，有时会导致返回的引用指向无效数据，
// 这种 情况下禁止它返回这样的引用。下面例子展示了一些合法的带有生命周期的函数：

#[test]
fn third() {
    // 一个拥有生命周期 `'a` 的输入引用，其中 `'a` 的存活时间
    // 至少与函数的一样长。
    fn print_one<'a>(x: &'a i32) {
        println!("`print_one`: x is {}", x);
    }

    // 可变引用同样也可能拥有生命周期。
    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }

    // 拥有不同生命周期的多个元素。对下面这种情形，两者即使拥有
    // 相同的生命周期 `'a` 也没问题，但对一些更复杂的情形，可能
    // 就需要不同的生命周期了。
    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("`print_multi`: x is {}, y is {}", x, y);
    }

    // 返回传递进来的引用也是可行的。
    // 但必须返回正确的生命周期。
    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }

    //fn invalid_output<'a>() -> &'a String { &String::from("foo") }
    // 上面代码是无效的：`'a` 存活的时间必须比函数的长。
    // 这里的 `&String::from("foo")` 将会创建一个 `String` 类型，然后对它取引用。
    // 数据在离开作用域时删掉，返回一个指向无效数据的引用。
    {
        let x = 7;
        let y = 9;

        print_one(&x);
        print_multi(&x, &y);

        let z = pass_x(&x, &y);
        print_one(z);

        let mut t = 3;
        add_one(&mut t);
        print_one(&t);
    }
}

// 方法 method
#[test]
fn fourth() {
    struct Owner(i32);

    impl Owner {
        fn add_one<'a>(&'a mut self) {
            self.0 += 1;
        }
        fn print<'a>(&'a self) {
            println!("print :{} ", self.0);
        }
    }

    {
        let mut owner = Owner(122);
        owner.add_one();
        owner.print();
    }
}

// struct 结构体
#[test]
fn fifth() {
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    {
        let x = 18;
        let y = 15;

        let single = Borrowed(&x);
        let double = NamedBorrowed { x: &x, y: &y };
        let reference = Either::Ref(&x);
        let number = Either::Num(y);

        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);
    }
}

// trait
#[test]
fn sixth() {
    // 带有生命周期标注的结构体。
    #[derive(Debug)]
    struct Borrowed<'a> {
        x: &'a i32,
    }

    // 给 impl 标注生命周期。
    impl<'a> Default for Borrowed<'a> {
        fn default() -> Self {
            Self { x: &10 }
        }
    }

    {
        let b: Borrowed = Default::default();
        println!("b is {:?}", b);
    }
}

// bounds 约束
// 就如泛型类型能够被约束一样，生命周期（它们本身就是泛型）也可以使用约束。
// : 字符 的意义在这里稍微有些不同，不过 + 是相同的。注意下面的说明：
//
// T: 'a：在 T 中的所有引用都必须比生命周期 'a 活得更长。
// T: Trait + 'a：T 类型必须实现 Trait trait，并且在 T 中的所有引用 都必须比 'a 活得更长。
// 下面例子展示了上述语法的实际应用：
#[test]
fn seventh() {
    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);
    // `Ref` 包含一个指向泛型类型 `T` 的引用，其中 `T` 拥有一个未知的生命周期
    // `'a`。`T` 拥有生命周期限制， `T` 中的任何*引用*都必须比 `'a` 活得更长。另外
    // `Ref` 的生命周期也不能超出 `'a`。

    // 一个泛型函数，使用 `Debug` trait 来打印内容。
    fn print<T>(t: T)
    where
        T: Debug,
    {
        println!("`print`: t is {:?}", t);
    }

    // 这里接受一个指向 `T` 的引用，其中 `T` 实现了 `Debug` trait，并且在 `T` 中的
    // 所有*引用*都必须比 `'a'` 存活时间更长。另外，`'a` 也要比函数活得更长。
    fn print_ref<'a, T>(t: &'a T)
    where
        T: Debug + 'a,
    {
        println!("`print_ref`: t is {:?}", t);
    }

    {
        let x = 7;
        let ref_x = Ref(&x);

        print_ref(&ref_x);
        print(ref_x);
    }
}

// 强制转换
// 一个较长的生命周期可以强制转成一个较短的生命周期，
// 使它在一个通常情况下不能工作 的作用域内也能正常工作。
// 强制转换可由编译器隐式地推导并执行，也可以通过声明不同
// 的生命周期的形式实现。
#[test]
fn eighth() {
    // 在这里，Rust 推导了一个尽可能短的生命周期。
    // 然后这两个引用都被强制转成这个生命周期。
    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }

    // `<'a: 'b, 'b>` 读作生命周期 `'a` 至少和 `'b` 一样长。
    // 在这里我们我们接受了一个 `&'a i32` 类型并返回一个 `&'b i32` 类型，这是
    // 强制转换得到的结果。
    fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }

    {
        let first = 2; // 较长的生命周期

        {
            let second = 3; // 较短的生命周期

            println!("The product is {}", multiply(&first, &second));
            println!("{} is the first", choose_first(&first, &second));
        };
    }
}

// static
//
// 'static 生命周期是可能的生命周期中最长的，它会在整个程序运行的时期中 存在。
// 'static 生命周期也可被强制转换成一个更短的生命周期。
// 有两种方式使变量 拥有 'static 生命周期，它们都把数据保存在可执行文件的只读内存区：
//
// 使用 static 声明来产生常量（constant）。
// 产生一个拥有 &'static str 类型的 string 字面量。
// 看下面的例子，了解列举到的各个方法：
#[test]
fn ninth() {
    static NUM: i32 = 18;

    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }
    {
        {
            // 产生一个 `string` 字面量并打印它：
            let static_string = "I'm in read-only memory";
            println!("static_string: {}", static_string);

            // 当 `static_string` 离开作用域时，该引用不能再使用，不过
            // 数据仍然存在于二进制文件里面。
        }

        {
            // 产生一个整型给 `coerce_static` 使用：
            let lifetime_num = 9;

            // 将对 `NUM` 的引用强制转换成 `lifetime_num` 的生命周期：
            let coerced_static = coerce_static(&lifetime_num);

            println!("coerced_static: {}", coerced_static);
        }

        println!("NUM: {} stays accessible!", NUM);
    }
}

// 省略

#[test]
fn tenth() {
    // `elided_input` 和 `annotated_input` 事实上拥有相同的签名，
    // `elided_input` 的生命周期会被编译器自动添加：
    fn elided_input(x: &i32) {
        println!("`elided_input`: {}", x)
    }

    fn annotated_input<'a>(x: &'a i32) {
        println!("`annotated_input`: {}", x)
    }

    // 类似地，`elided_pass` 和 `annotated_pass` 也拥有相同的签名，
    // 生命周期会被隐式地添加进 `elided_pass`：
    fn elided_pass(x: &i32) -> &i32 {
        x
    }

    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
        x
    }

    {
        let x = 3;

        elided_input(&x);
        annotated_input(&x);

        println!("`elided_pass`: {}", elided_pass(&x));
        println!("`annotated_pass`: {}", annotated_pass(&x));
    }
}
