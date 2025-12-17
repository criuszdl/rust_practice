// fn main() {
//     println!("main function is running !");
//     // another_function(5);
//     // fn another_function(param: i32) {
//     //     // println!("custom function is running , and speed is {}", param);
//     //     println!("custom function is running , and speed is {param}");
//     // }
//
//
//     // another_function2(5,'h');
//     // fn another_function2(param: i32,s:char) {
//     //     println!("custom function is running , and speed is {param} and {s}");
//     // }
//
//     let x = {
//         let b = 5;
//         b + 6
//     };
//     println!("x is {}", x);
// }


// fn five() -> i32 {
//     5
// }
//
// fn six() -> i32 {
//     6
// }
// fn main() {
//     let x = five();
//     let y = six();
//     println!("The value of x is: {x},{y}");
// }

fn plus_one(x: i32) -> i32 {
  return x + 1;
}

fn main() {
  let x = plus_one(5);
  println!("{}", x);
}
