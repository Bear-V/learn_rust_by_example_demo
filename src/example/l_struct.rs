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
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的参数
#[allow(dead_code)]
#[derive(Debug)]
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

    let point = Point { x: 2.2, y: 2.4 };

    println!("point coordinates: {} {}", point.x, point.y);

    let new_point = Point { y: 0.2, ..point };

    println!("second point: {} {}", new_point.x, new_point.y);

    let Point { x: my_x, y: my_y } = point;

    let rectangle = Rectangle {
        p1: Point { x: my_x, y: my_y },
        p2: Point { x: 0.2, y: 0.4 },
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {} and {}", integer, decimal);

    fn rect_area(rectangle: Rectangle) {
        let Point { x: x_1, y: y_1 } = rectangle.p1;
        let Point { x: x_2, y: y_2 } = rectangle.p2;

        let height = x_1 - x_2;
        let width = y_1 - y_2;

        let area = height * width;
        println!("面积是:{}", area);
    }

    rect_area(rectangle);

    fn square(point: Point, length: f32) -> Rectangle {
        let new_point = Point {
            x: point.x + length,
            y: point.y + length,
        };

        Rectangle {
            p1: point,
            p2: new_point,
        }
    }

    let new_rec = square(point,0.5);

    println!("new rectangle is {:?}",new_rec);
}
