#[test]
#[allow(unused_doc_comments)]
fn one() {
    // 单行注释

    /*
    块注释
     */

    /// 文档注释 用来生成帮助文档
    //！注释所属于的项
    let x = 5 + /* 90 +*/ 5;

    println!("is x 10 or 100? x = {}", x);
}
