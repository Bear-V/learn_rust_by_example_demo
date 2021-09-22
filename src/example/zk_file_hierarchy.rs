/**
 * @Author: ZZX
 * @Description: 文件分层
 * @Date: create in 2021/9/16 1:43 下午
 */
// 此声明将会查找名为 `my.rs` 或 `my/mod.rs` 的文件，并将该文件的内容放到
// 此作用域中一个名为 `my` 的模块里面。

use crate::example::my;

fn function() {
    println!("called `function()`");
}

#[test]
fn one() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
