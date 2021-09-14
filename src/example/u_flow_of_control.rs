/**
 * @Author: ZZX
 * @Description: 流程控制 if/else for loop while
 * @Date: create in 2021/9/14 10:39 上午
 */

#[test]
fn one() {
    // if / else
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero ", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number , increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, half the number");
        n / 2
    };

    println!("{}->{}", n, big_n);
}

#[test]
fn two() {
    // loop

    let mut count = 0u32;
    println!("let's count until infinity");

    loop {
        count += 1;
        if count == 3 {
            println!("three");

            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK");
            break;
        }
    }
}

#[test]
#[allow(unreachable_code)]
#[allow(unused_labels)]
fn third() {
    'outer: loop {
        println!("entered the outer loop");

        'inner: loop {
            println!("entered the inner loop");

            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}

#[test]
fn fourth() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20)
}

fn foo(n: i32) {
    if n % 15 == 0 {
        println!("fizzbuzz");
    } else if n % 3 == 0 {
        println!("fizz");
    } else if n % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

#[test]
fn fifth() {
    // while
    let mut n = 1;

    // 当 `n` 小于 101 时循环
    while n < 101 {
        foo(n);

        // 计数器值加 1
        n += 1;
    }
}

#[test]
fn sixth() {
    // for
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..101 {
        foo(n);
    }
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..=100 {
        foo(n);
    }
}
