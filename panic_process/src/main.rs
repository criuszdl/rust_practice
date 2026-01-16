// fn main() {
//     //直接显示panic
//     // panic!("something went wrong");
//
//     //隐士触发 + 观察 backTrace
//     // let v = vec![1,2,3,4,5,6,7,8,9,10];
//     // v[99];
// }

//用Result处理可恢复错误
// use std::fs::File;
// use std::io::ErrorKind;
//
// fn main() {
//     // let my_file_result =  File::open("hello.txt");
//     //笼统区分
//     // let my_file = match my_file_result {
//     //     Ok(file) => file,
//     //     Err(error) => panic!("Problem opening the file: {error:?}"),
//     // };
//
//     //详细区分各种panic
//     // let _my_file = match my_file_result {
//     //     Ok(file) => file,
//     //     Err(error) => match error.kind() {
//     //         ErrorKind::NotFound => match File::create("hello.txt") {
//     //             Ok(file) => file,
//     //             Err(e) => panic!("Problem creating the file: {:?}", e),
//     //         },
//     //         _ => panic!("Problem opening the file: {:?}", error),
//     //     }
//     // };
//
//     // let my_file2 = File::open(Path::new("hello.txt")).unwrap_or_else(|why| {
//     //     if why.kind() == ErrorKind::NotFound {
//     //         File::create("hello.txt").unwrap_or_else(|why| {
//     //             panic!("couldn't create the file:{:#?}",why);
//     //         })
//     //     }else {
//     //         panic!("problem opening the file:{:#?}",why);
//     //     }
//     // });
//
//
// }


// use std::fs::File;
// fn main() {
//     // let file = File::open("hello_world.txt").unwrap();
//     let file = File::open("hello_world.txt").expect("hello_world.txt should be included in this practice project.");
// }

// use std::fs;
// use std::fs::File;
// use std::io::{self,Read};
//
// fn read_username_from_file() -> Result<String,io::Error>{
//     let username_file_result = File::open("hello_world.txt");
//
//     let username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username){
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
//
// fn read_username_from_file2() -> Result<String,io::Error>{
//     let username_file = File::open("haha.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(username)?;
//     Ok(username)
// }
//
// fn read_username_from_file3() -> Result<String,io::Error>{
//     let mut username = String::new();
//     File::open("haha.txt")?.read_to_string(username)?;
//     Ok(username)
// }
//
// fn read_username_from_file4() -> Result<String,io::Error>{
//     fs::read_to_string("nihao.txt")
// }

// use std::fs::File;
// fn main() {
//     let greeting_file = File::open("hello.txt")?;
// }
//
// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last()
// }

// use std::error::Error;
// use std::fs::File;
//
// fn main() -> Result<(),Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;
//     Ok(())
// }

// fn guess_age(input: &str) -> i32 {
//     input.parse().unwrap()
// }
// fn guess_age(input: &str) -> Result<i32,> {
//     input.parse()
// }

fn main() {
    loop{
        let guess_number: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess_number < 1 || guess_number > 100 {
            println!("猜测的数字应该在1-100之间");
            continue;
        }

        //开始match  匹配判断

    }
}


