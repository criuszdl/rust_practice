// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//
//     println!("the area of rectangle is {} square pixels.", area(width1, height1));
// }
//
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//使用元组重构
// fn main() {
//     let rect = (30, 50);
//     println!("the area of rectangle is {} square pixels.", area_tup(rect));
// }
//
// fn area_tup(dimensions:(u32,u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

//使用结构体重构
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let rect = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     //借用结构体而不是获取它的所有权, main 函数就可以保持 rect1 的所有权并继续使用它
//     println!("the area of rectangle is {} square pixels.", area_s(&rect));
//     //如果上一步不加 &，所有权会 移动 到函数内，随者函数执行完毕后}失效。
//     println!("the width is {} .", rect.width);
// }
//
// fn area_s(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

//使用trait增加功能
#[allow(dead_code)] //临时关闭特定项的警告
// #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        // width: dbg!(30 * 6),
        height: 50,
    };
    // println!("rect1 is {rect1:?}");
    println!("rect1 is  {rect1:#?}");

    // dbg!(&rect1);
}