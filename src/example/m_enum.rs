/**
 * @Author: ZZX
 * @Description: 枚举
 * @Date: create in 2021/9/9 23:41
 */

enum WebEvent {
    // 单元结构体 称为unit-like or unit
    PageLoad,
    PageUnload,
    // 元组结构体
    KeyPress(char),
    Paste(String),
    // 普通结构体
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\" .", s),
        WebEvent::Click { x, y } => println!("clicked at x = {} , y = {}", x, y),
    }
}

#[test]
fn one() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

#[allow(dead_code)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

#[test]
fn two() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let _x_1 = Operations::Add;
    let y_1 = Operations::Subtract;

    let x = 32;
    let y = 22;

    let v = VeryVerboseEnumOfThingsToDoWithNumbers::run(&y_1, x, y);
    println!("{}",v);
}
