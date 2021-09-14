/**
 * @Author: ZZX
 * @Description: match 匹配
 * @Date: create in 2021/9/14 3:06 下午
 */

#[test]
fn one() {
    let number = 13;

    println!("tell me about {}", number);

    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        13..=19 => println!("a teen"),
        _ => println!("ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{}->{}", boolean, binary);
}
