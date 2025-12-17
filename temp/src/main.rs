fn main() {
    // let mut s = String::from("hello");
    // s.push_str(", world!"); // push_str() 在字符串后追加字面值
    // println!("{s}"); // 将打印 `hello, world!`

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{},world",s1); // 报错

    // let s2 = s1.clone();
    // println!("{}", s1);
    // println!("{}", s2);

    // let x = 5;
    // let y = x;
    // println!("{}", x);
    // println!("{}", y);

    // let s = String::from("hello");
    //
    // takes_ownership(s);
    // // println!("{}", s);
    //
    // let x = 5 ;
    // makes_copy(x);
    // println!("{}", x);

    let s = String::from("hello");
    let (s2,len) =  calculate_length(s);
    println!("The length of '{}' is {}.", s2, len);
}
// fn takes_ownership(some_string: String){
//     println!("{}", some_string);
//     // let y = some_string;
//     // println!("{}", y);
// }
//
// fn makes_copy(some_integer: i32){
//     println!("{}", some_integer);
// }

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}