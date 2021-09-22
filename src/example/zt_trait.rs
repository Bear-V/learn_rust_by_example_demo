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
            Sheep {
                name,
                naked: false,
            }
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
        let mut dolly:Sheep = Animal::new("Dolly");
        // 试一试 ^ 移除类型标注。

        dolly.talk();
        dolly.shear();
        dolly.talk();
    }

}
