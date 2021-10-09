use std::ops;

/**
* @Author: ZZX
* @Description: trait
   trait 是对未知类型 Self 定义的方法集。
   该类型也可以访问同一个 trait 中定义的 其他方法。
   对任何数据类型都可以实现 trait
* @Date: create in 2021/9/22 3:33 下午
*/

#[test]
fn one() {
    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    trait Animal {
        fn new(name: &'static str) -> Self;

        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        fn talk(&self) {
            println!("{} say {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                println!("{} is already naked", self.name());
            } else {
                println!("{} gets a haircut", self.name);
                self.naked = true
            }
        }
    }

    // 对 `Sheep` 实现 `Animal` trait。
    impl Animal for Sheep {
        // `Self` 是实现者类型：`Sheep`。
        fn new(name: &'static str) -> Sheep {
            Sheep { name, naked: false }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }

        // 默认 trait 方法可以重载。
        fn talk(&self) {
            // 例如我们可以增加一些安静的沉思。
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    {
        // 这种情况需要类型标注。
        let mut dolly: Sheep = Animal::new("Dolly");
        // 试一试 ^ 移除类型标注。

        dolly.talk();
        dolly.shear();
        dolly.talk();
    }
}

// derive
//通过 #[derive] 属性，编译器能够提供某些 trait 的基本实现。
// 如果 需要更复杂的行为，这些 trait 也可以手动实现。
//
// 下面以下是可以自动派生的 trait：
//
// 比较 trait:
// Eq, PartialEq,
// Ord, PartialOrd
// Clone, 用来从 &T 创建副本 T。
// Copy，使类型具有 “复制语义”（copy semantics）而非 “移动语义”（move semantics）。
// Hash，从 &T 计算哈希值（hash）。
// Default, 创建数据类型的一个空实例。
// Debug，使用 {:?} formatter 来格式化一个值

#[test]
fn two() {
    // `Centimeters`，可以比较的元组结构体
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    // `Inches`，可以打印的元组结构体
    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    // `Seconds`，不带附加属性的元组结构体
    struct Seconds(i32);
    {
        let _one_second = Seconds(1);

        // 报错：`Seconds` 不能打印；它没有实现 `Debug` trait
        //println!("One second looks like: {:?}", _one_second);
        // 试一试 ^ 取消此行注释

        // 报错：`Seconds`不能比较；它没有实现 `PartialEq` trait
        //let _this_is_true = (_one_second == _one_second);
        // 试一试 ^ 取消此行注释

        let foot = Inches(12);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);

        let cmp = if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

        println!("One foot is {} than one meter.", cmp);
    }
}

// 使用 dyn 返回 trait
//
// Rust 编译器需要知道每个函数的返回类型需要多少空间。这意味着所有函数都必须返回一个具体类型。
// 与其他语言不同，如果你有个像 Animal 那样的的 trait，
// 则不能编写返回 Animal 的函数，因为其不同的实现将需要不同的内存量。
//
// 但是，有一个简单的解决方法。相
// 比于直接返回一个 trait 对象，我们的函数返回一个包含一些 Animal
// 的 Box。box 只是对堆中某些内存的引用。
// 因为引用的大小是静态已知的，并且编译器可以保证引用指向已分配的堆 Animal，
// 所以我们可以从函数中返回 trait！
//
// 每当在堆上分配内存时，Rust 都会尝试尽可能明确。
//
// 因此，如果你的函数以这种方式返回指向堆的 trait 指针，
// 则需要使用 dyn 关键字编写返回类型，例如 Box<dyn Animal>。
#[test]
fn third() {
    struct Sheep {}
    struct Cow {}

    trait Animal {
        // 实例方法签名
        fn noise(&self) -> &'static str;
    }

    // 实现 `Sheep` 的 `Animal` trait。
    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "baaaaah!"
        }
    }

    // 实现 `Cow` 的 `Animal` trait。
    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "moooooo!"
        }
    }

    // 返回一些实现 Animal 的结构体，但是在编译时我们不知道哪个结构体。
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    {
        let random_number = 0.234;
        let animal = random_animal(random_number);
        println!(
            "You've randomly chosen an animal, and it says {}",
            animal.noise()
        );
    }
}

//运算符重载
//
// 在 Rust 中，很多运算符可以通过 trait 来重载。
// 也就是说，这些运算符可以根据它们的 输入参数来完成不同的任务。
// 这之所以可行，是因为运算符就是方法调用的语法糖。
// 例 如，a + b 中的 + 运算符会调用 add 方法（也就是 a.add(b)）。
// 这个 add 方 法是 Add trait 的一部分。
// 因此，+ 运算符可以被任何 Add trait 的实现者使用。
//
// 会重载运算符的 trait（比如 Add 这种）可以在这里查看。
#[test]
fn fifth() {
    struct Foo;
    struct Bar;

    #[derive(Debug)]
    struct FooBar;

    #[derive(Debug)]
    struct BarFoo;

    // `std::ops::Add` trait 用来指明 `+` 的功能，
    // 这里我们实现 `Add<Bar>`，它是用于
    // 把对象和 `Bar` 类型的右操作数（RHS）加起来的 `trait`。
    // 下面的代码块实现了 `Foo + Bar = FooBar` 这样的运算。
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            println!("> Foo.add(Bar) was called");

            FooBar
        }
    }

    // 通过颠倒类型，我们实现了不服从交换律的加法。
    // 这里我们实现 `Add<Foo>`，
    // 它是用于把对象和 `Foo` 类型的右操作数加起来的 trait。
    // 下面的代码块实现了 `Bar + Foo = BarFoo` 这样的运算。
    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;

        fn add(self, _rhs: Foo) -> BarFoo {
            println!("> Bar.add(Foo) was called");

            BarFoo
        }
    }

    {
        println!("Foo + Bar = {:?}", Foo + Bar);
        println!("Bar + Foo = {:?}", Bar + Foo);
    }
}

// Drop
//
// Drop trait 只有一个方法：drop，当对象离开作用域时会自动调用该 方法。
// Drop trait 的主要作用是释放实现者的实例拥有的资源。
//
// Box，Vec，String，File，
// 以及 Process 是一些实现了 Drop trait 来释放 资源的类型。
// Drop trait 也可以为任何自定义数据类型手动实现。
//
// 下面示例给 drop 函数增加了打印到控制台的功能，
// 用于宣布它在什么时候被调用。 when it is called.）
#[test]
fn sixth() {
    struct Droppable {
        name: &'static str,
    }

    // 这个简单的 `drop` 实现添加了打印到控制台的功能。
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }

    {
        let _a = Droppable { name: "a" };

        // 代码块 A
        {
            let _b = Droppable { name: "b" };

            // 代码块 B
            {
                let _c = Droppable { name: "c" };
                let _d = Droppable { name: "d" };

                println!("Exiting block B");
            }
            println!("Just exited block B");

            println!("Exiting block A");
        }
        println!("Just exited block A");

        // 变量可以手动使用 `drop` 函数来销毁。
        // drop(_a);
        // 试一试 ^ 将此行注释掉。

        println!("end of the main function");

        // `_a` *不会*在这里再次销毁，因为它已经被（手动）销毁。
    }
}
