// //定义
// enum IpAddrKind{
//     V4,
//     V6,
// }
//
// fn main() {
//     //创建IpAddrKind 两个不同变体的实例
//     //枚举的变体位于其标识符的命名空间中，并使用两个冒号分开
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//
//     //函数调用
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }
//
// //定义一个函数来接收任何 IpAddrKind 类型的参数
// fn route(ip_kind : IpAddrKind) {
//
// }

// ===================================================
//1. 使用结构体 Struct 存储实际 IP 地址数据
// enum IpAddrKind{
//     V4,
//     V6,
// }
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// fn main() {
//     //使用了一个结构体来将 kind 和 address 打包在一起，现在枚举变体就与值相关联.
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
//
// }


//2. 枚举关联 String 值
// enum IpAddrKind{
//     V4(String),
//     V6(String),
// }
//
// fn main() {
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
// }

//3. 将 V4 地址存储为四个 u8 值而 V6 地址仍然表现为一个 String
// enum IpAddr{
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

//4. 包含多种类型的枚举
// enum Message {
//     Quit,
//     Move {x : i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// impl Message {
//     fn call(&self) {
//         println!("Calling message...");
//     }
// }
//
// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

//Option
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    // let some_number = Some(5);
    // let some_string = Some("a string");
    //
    // let absent_number: Option<u32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
}






