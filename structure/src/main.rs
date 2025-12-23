fn main() {

    //实例化（不可变实例）
    // let user1 = User{
    //     active: true,
    //     username: String::from("crius"),
    //     email:String::from("example@outlook.com"),
    //     sign_in_count:0,
    // };
    //
    // println!(" the user1 active is {}", user1.active);
    // println!(" the user1 username is {}", user1.username);
    // println!(" the user1 email is {}", user1.email);
    // println!(" the user1 sign_in_count is {}", user1.sign_in_count);

    //实例化（可变实例:mut）
    // let mut user2 = User{
    //     active: true,
    //     username: String::from("crius"),
    //     email:String::from("example@outlook.com"),
    //     sign_in_count:0,
    // };
    //
    // user2.sign_in_count= 2;
    // println!("the user1 sign_in_count is {}", user2.sign_in_count);

    // let user3 = create_user("example@123.com","lisi");
    //
    // // 普通方法
    // let user4 = User{
    //     active:false,
    //     username:user3.username,
    //     email:user3.email,
    //     sign_in_count:user3.sign_in_count,
    // };
    //
    // // 结构体更新语法（struct update syntax）
    // let user4 = User{
    //     email: String::from("another@example.com"),
    //     //..user3 必须放在最后，以指定其余的字段应从 user3 的相应字段中获取其值
    //     ..user3
    // };

    //元组结构体
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    //解构元组结构体时必须写明结构体的类型。
    let Point(x,y,z) = origin;

}

// fn create_user(email: String, username: String) -> User {
//     User{
//         active: true,
//         //字段初始化简写语法（fi eld init shorthand）,参数与字段相同，直接写一次即可。
//         username,
//         //因为 email 字段与 email 参数有着相同的名称，则只需编写 email 而不是 email: email。
//         email,
//         sign_in_count:0,
//     }
// }


//结构体定义
struct User{
    //各个字段
    active:bool,
    username:String,
    email:String,
    sign_in_count:i32,
}

struct Color(i32, i32, i32);
struct Point(f64, f64, f64);
