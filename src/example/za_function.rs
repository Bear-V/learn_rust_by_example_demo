/**
 * @Author: ZZX
 * @Description: 函数
 * @Date: create in 2021/9/14 5:12 下午
 */

#[test]
fn one() {
    fizzbuzz_to(100);

    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }

        lhs % rhs == 0
    }
    // 一个 “不” 返回值的函数。实际上会返回一个单元类型 `()`。
    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // 当函数返回 `()` 时，函数签名可以省略返回类型
    fn fizzbuzz_to(n: u32) {
        for n in 1..n + 1 {
            fizzbuzz(n);
        }
    }
}
