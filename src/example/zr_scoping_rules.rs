/**
* @Author: ZZX
* @Description: 作用域规则
   作用域在所有权（ownership）、借用（borrow）和生命周期（lifetime）中起着重要 作用。
   也就是说，作用域告诉编译器什么时候借用是合法的、什么时候资源可以释放、以及 变量何时被创建或销毁。
* @Date: create in 2021/9/17 3:03 下午
*/

///RAII
//
// Rust 的变量不只是在栈中保存数据：它们也占有资源，比如 Box<T> 占有 堆（heap）中的内存。
// Rust 强制实行 RAII（Resource Acquisition Is Initialization，资源获取即初始化），
// 所以任何对象在离开作用域时，它的析构 函数（destructor）就被调用，然后它占有的资源就被释放。
//
// 这种行为避免了资源泄漏（resource leak），
// 所以你再也不用手动释放内存或者担心 内存泄漏（memory leak）！
// 下面是个快速入门示例：
#[test]
fn one() {
    fn crate_box() {
        let _box1 = Box::new(3i32);
    }

    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);
    }

    for _ in 0u32..1_000 {
        crate_box();
    }

    struct ToDrop;

    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped");
        }
    }

    {
        let _x = ToDrop;
        println!("Made a ToDrop");
    }
}

///所有权和移动
//
// 因为变量要负责释放它们拥有的资源，所以资源只能拥有一个所有者。
// 这也防止了 资源的重复释放。注意并非所有变量都拥有资源（例如引用）。
//
// 在进行赋值（let x = y）或通过值来传递函数参数（foo(x)）的时候，
// 资源 的所有权（ownership）会发生转移。按照 Rust 的说法，这被称为资源 的移动（move）。
//
// 在移动资源之后，原来的所有者不能再被使用，
// 这可避免悬挂指针（dangling pointer）的产生。
#[test]
fn two() {
    fn destroy_box(c: Box<i32>) {
        println!("destroying a box that contains {}", c);
    }

    {
        let x = 5u32;
        let y = x;
        println!("x is {} , y is {}", x, y);
        println!("x is {} , y is {}", x, y);

        // `a` 是一个指向堆分配的整数的指针
        let a = Box::new(5i32);

        println!("a contains: {}", a);

        // *移动* `a` 到 `b`
        let b = a;
        // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向
        // 同一个堆分配的数据，但是现在是 `b` 拥有它。

        // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存。
        // println!("a contains: {}", a);
        // 试一试 ^ 去掉此行注释

        // 此函数从 `b` 中取得堆分配的内存的所有权
        destroy_box(b);

        // 此时堆内存已经被释放，这个操作会导致解引用已释放的内存，而这是编译器禁止的。
        // 报错！和前面出错的原因一样。
        // println!("b contains: {}", b);
        // 试一试 ^ 去掉此行注释
    }
}

// 可变性
#[test]
fn third() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // 可变性错误
    //*immutable_box = 4;

    // *移动* box，改变所有权（和可变性）
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // 修改 box 的内容
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}

/// 部分移动
/// 在单个变量的解构内，可以同时使用 by-move 和 by-reference 模式绑定。
/// 这样做将导致变量的部分移动（partial move），
/// 这意味着变量的某些部分将被移动，而其他部分将保留。
/// 在这种情况下，后面不能整体使用父级变量，但是仍然可以使用只引用（而不移动）的部分。
#[test]
fn fourth() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: "Alice".to_string(),
        age: 20,
    };

    let Person { name, ref age } = person;
    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // 报错！部分移动值的借用：`person` 部分借用产生
    //println!("The person struct is {:?}", person);

    // `person` 不能使用，但 `person.age` 因为没有被移动而可以继续使用
    println!("The person's age from person struct is {}", person.age);
}

// 借用
//
// 多数情况下，我们更希望能访问数据，同时不取得其所有权。
// 为实现这点，Rust 使用 了借用（borrowing）机制。
// 对象可以通过引用（&T）来传递，从而取代通过 值（T）来传递。
//
// 编译器（通过借用检查）静态地保证了引用总是指向有效的对象。
// 也就是说，当存在 引用指向一个对象时，该对象不能被销毁。
#[test]
fn fifth() {
    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("destroying box that contains {}", boxed_i32);
    }

    fn borrow_i32(borrowed_i32: &i32) {
        println!("this int is :{}", borrowed_i32);
    }

    {
        let boxed_i32 = Box::new(5i32);
        let stacked_i32 = 6_i32;

        // 借用了 box 的内容，但没有取得所有权，所以 box 的内容之后可以再次借用。
        // 译注：请注意函数自身就是一个作用域，因此下面两个函数运行完成以后，
        // 在函数中临时创建的引用也就不复存在了。
        borrow_i32(&boxed_i32);
        borrow_i32(&stacked_i32);

        {
            // 取得一个对 box 中数据的引用
            let _ref_to_i32: &i32 = &boxed_i32;

            // 报错！
            // 当 `boxed_i32` 里面的值之后在作用域中被借用时，不能将其销毁。
            // eat_box_i32(boxed_i32);
            // 改正 ^ 注释掉此行

            // 在 `_ref_to_i32` 里面的值被销毁后，尝试借用 `_ref_to_i32`
            //（译注：如果此处不借用，则在上一行的代码中，eat_box_i32(boxed_i32)可以将 `boxed_i32` 销毁。）
            borrow_i32(_ref_to_i32);
            // `_ref_to_i32` 离开作用域且不再被借用。
        }

        // `boxed_i32` 现在可以将所有权交给 `eat_i32` 并被销毁。
        //（译注：能够销毁是因为已经不存在对 `boxed_i32` 的引用）
        eat_box_i32(boxed_i32);
    }
}

//可变性
//
// 可变数据可以使用 &mut T 进行可变借用。
// 这叫做可变引用（mutable reference），它使借用者可以读/写数据。
// 相反，&T 通过不可变引用（immutable reference）来借用数据，
// 借用者可以读数据而不能更改数据：

#[test]
fn sixth() {
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct Book {
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    fn borrow_book(book: &Book) {
        println!(
            "I immutably borrowed {} - {} edition",
            book.title, book.year
        );
    }

    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edition", book.title, book.year);
    }

    {
        // 创建一个名为 `immutabook` 的不可变的 Book 实例
        let immutabook = Book {
            // 字符串字面量拥有 `&'static str` 类型
            author: "Douglas Hofstadter",
            title: "Gödel, Escher, Bach",
            year: 1979,
        };

        // 创建一个 `immutabook` 的可变拷贝，命名为 `mutabook`
        let mut mutabook = immutabook;

        // 不可变地借用一个不可变对象
        borrow_book(&immutabook);

        // 不可变地借用一个可变对象
        borrow_book(&mutabook);

        // 可变地借用一个可变对象
        new_edition(&mut mutabook);

        // 报错！不能可变地借用一个不可变对象
        // new_edition(&mut immutabook);
        // 改正 ^ 注释掉此行
    }
}

// 别名使用
// 数据可以多次不可变借用，但是在不可变借用的同时，原始数据不能使用可变借用。
// 或者说，同一时间内只允许一次可变借用。
// 仅当最后一次使用可变引用之后，原始数据才可以再次借用
#[test]
fn seventh() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    {
        let mut point = Point { x: 0, y: 0, z: 0 };

        let borrowed_point = &point;
        let another_borrow = &point;

        println!(
            "Point has coordinates: ({},{},{})",
            borrowed_point.x, borrowed_point.y, point.z
        );
        // 报错！`point` 不能以可变方式借用，因为当前还有不可变借用。
        // let mutable_borrow = &mut point;
        // TODO ^ 试一试去掉此行注释

        // 被借用的值在这里被重新使用
        println!(
            "Point has coordinates: ({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z
        );

        // 不可变的引用不再用于其余的代码，因此可以使用可变的引用重新借用。
        let mutable_borrow = &mut point;

        // 通过可变引用来修改数据
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // 报错！不能再以不可变方式来借用 `point`，因为它当前已经被可变借用。
        // let y = &point.y;
        // TODO ^ 试一试去掉此行注释

        // 报错！无法打印，因为 `println!` 用到了一个不可变引用。
        // println!("Point Z coordinate is {}", point.z);
        // TODO ^ 试一试去掉此行注释

        // 正常运行！可变引用能够以不可变类型传入 `println!`
        println!(
            "Point has coordinates: ({}, {}, {})",
            mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
        );

        // 可变引用不再用于其余的代码，因此可以重新借用
        let new_borrowed_point = &point;
        println!(
            "Point now has coordinates: ({}, {}, {})",
            new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
        );
    }
}

//ref 模式
//
// 在通过 let 绑定来进行模式匹配或解构时，ref 关键字可用来创建结构体/元组的 字段的引用。
// 下面的例子展示了几个实例，可看到 ref 的作用
#[test]
fn eighth() {
    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    {
        let c = 'Q';

        // 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号。
        let ref ref_c1 = c;
        let ref_c2 = &c;

        println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

        let point = Point { x: 0, y: 0 };

        // 在解构一个结构体时 `ref` 同样有效。
        let _copy_of_x = {
            // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
            let Point {
                x: ref ref_to_x,
                y: _,
            } = point;

            // 返回一个 `point` 的 `x` 字段的拷贝。
            *ref_to_x
        };

        // `point` 的可变拷贝
        let mut mutable_point = point;

        {
            // `ref` 可以与 `mut` 结合以创建可变引用。
            let Point {
                x: _,
                y: ref mut mut_ref_to_y,
            } = mutable_point;

            // 通过可变引用来改变 `mutable_point` 的字段 `y`。
            *mut_ref_to_y = 1;
        }

        println!("point is ({}, {})", point.x, point.y);
        println!(
            "mutable_point is ({}, {})",
            mutable_point.x, mutable_point.y
        );

        // 包含一个指针的可变元组
        let mut mutable_tuple = (Box::new(5u32), 3u32);

        {
            // 解构 `mutable_tuple` 来改变 `last` 的值。
            let (_, ref mut last) = mutable_tuple;
            *last = 2u32;
        }

        println!("tuple is {:?}", mutable_tuple);
    }
}

