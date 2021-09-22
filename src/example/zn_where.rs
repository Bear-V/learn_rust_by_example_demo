/**
 * @Author: ZZX
 * @Description: where 分局
 * @Date: create in 2021/9/16 3:10 下午
 */

// impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
//
// // 使用 `where` 从句来表达约束
// impl<A, D> MyTrait<A, D> for YourType
// where
//     A: TraitB + TraitC,
//     D: TraitE + TraitF,
// {
// }

#[test]
fn one() {
    use std::fmt::Debug;

    trait PrintInOption {
        fn print_in_option(self);
    }

    // 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
    // 或着改用另一种间接的方法。
    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
        // 否则我们会给出错误的约束。
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
