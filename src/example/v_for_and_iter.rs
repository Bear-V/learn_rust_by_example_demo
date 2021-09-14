/**
 * @Author: ZZX
 * @Description:  for 和 迭代器
 * @Date: create in 2021/9/14 2:15 下午
 */

// iter() 为借用 还存在names内

#[test]
fn one() {
    let names = vec!["bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("there is a rustacean among us!"),
            _ => println!("hello {}", name),
        }
    }
    println!("{:?}", names);
}

// into_iter() 修改了所有权 再循环中 被 直接使用了
#[test]
fn two() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("{:?}",names);
}

// iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改
#[test]
fn third() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}