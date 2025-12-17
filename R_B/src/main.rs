// fn main() {
//     let s1 = String::from("hello");
//
//     let length  =  calculate_length(&s1);
//
//     println!("The length of '{}' is {}.", s1, length);
//
// }
//
// fn calculate_length(s: &String) ->usize{
//     s.len()
// }

// 改变引用，报错
// fn main() {
//     let s1 = String::from("hello");
//     change(&s1);
// }
//
// fn change(some_string: &String){
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s1 = String::from("hello");
//     change(&mut s1);
// }
//
// fn change(some_string: &mut String){
//     some_string.push_str(", world");
// }

fn main() {
    let my_string = String::from("hello world");
    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);
    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
}