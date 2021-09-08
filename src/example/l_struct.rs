/**
* @Author: ZZX
* @Description: 自定义类型，结构体
   元组结构体（tuple struct），事实上就是具名元组而已。
   经典的 C 语言风格结构体（C struct）。
   单元结构体（unit struct），不带字段，在泛型中很有用。
* @Date: create in 2021/9/7 3:41 下午
*/

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// 单元结构体
struct Nil;

// 元组结构体
struct Pair(i32, f32);

// 带两个字段的接口体
#[allow(dead_code)]
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的参数
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

#[test]
fn one() {
    let name = "Bear";
    let age = 30;
    let bear = Person { name, age };

    println!("{:?}", bear);

    let point = Point { x: 0.3, y: 0.4 };

    println!("point coordinates: {} {}", point.x, point.y);

    let new_point = Point { x: 0.1, ..point };

    println!("second point: {} {}", new_point.x, new_point.y);

    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: point,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer,decimal) = pair;

    println!("pair contains {} and {}",integer,decimal);

}