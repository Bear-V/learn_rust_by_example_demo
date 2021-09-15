/**
 * @Author: ZZX
 * @Description: 闭包的例子
 * @Date: create in 2021/9/15 2:38 下午
 */

#[test]
#[allow(array_into_iter)]
fn one() {
    // pub trait Iterator {
    //     // 被迭代的类型。
    //     type Item;
    //
    //     // `any` 接受 `&mut self` 参数（译注：回想一下，这是 `self: &mut Self` 的简写）
    //     // 表明函数的调用者可以被借用和修改，但不会被消耗。
    //     fn any<F>(&mut self, f: F) -> bool
    //     where
    //         // `FnMut` 表示被捕获的变量最多只能被修改，而不能被消耗。
    //         // `Self::Item` 指明了被捕获变量的类型（译注：是迭代器的元素本身的类型）
    //         F: FnMut(Self::Item) -> bool,
    //     {
    //     }
    //
    //     // 译注：原文说 `Self::Item` 表明变量是通过值传递给闭包的，这是说错了。
    //     // `FnMut` 就表示闭包只能通过引用捕获变量。把类型为 `T` 的变量作为闭包
    //     // 的参数不代表闭包会拿走它的值，也可能是拿走它的引用。
    // }

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 的 `iter()` 举出 `&i32`。（通过用 `&x` 匹配）把它解构成 `i32`。
    // 译注：注意 `any` 方法会自动地把 `vec.iter()` 举出的迭代器的元素一个个地
    // 传给闭包。因此闭包接收到的参数是 `&i32` 类型的。
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // 对 vec 的 `into_iter()` 举出 `i32` 类型。无需解构。
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32`。
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}

#[test]
fn two() {
    // pub trait Iterator {
    //     // 被迭代的类型。
    //     type Item;
    //
    //     // `find` 接受 `&mut self` 参数，表明函数的调用者可以被借用和修改，
    //     // 但不会被消耗。
    //     fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
    //     // `FnMut` 表示被捕获的变量最多只能被修改，而不能被消耗。
    //     // `&Self::Item` 指明了被捕获变量的类型（译注：是对迭代器元素的引用类型）
    //         P: FnMut(&Self::Item) -> bool {}
    // }

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1 {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec1 {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的 `iter()` 举出 `&i32`。
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // 对数组的 `into_iter()` 通常举出 `&i32``。
    println!(
        "Find 2 in array2: {:?}",
        array2.iter().find(|&&x| x == 2)
    );
}
