fn main() {

    //第一种方法，直接创建并指定数据类型
    // let mut v:Vec<i32> = Vec::new();

    //第二种使用vec!宏
    // let v2 = vec![1,2,3];

    //更新
    // v.push(10);
    // v.push(1);
    // v.push(3);
    // v.push(4);

    // let v = vec![1, 2, 3, 4, 5];
    // let third = &v[2];
    // println!("the third element is {third}");
    //
    // let fourth = &v.get(100);
    // let fourth = &v.get(5);
    // match fourth {
    //     Some(element) => println!("fourth element is {}", element),
    //     None => println!("There is no fourth element."),
    // }

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("the first element is {:?}", first);

    //遍历vector

    // let v = vec![1, 2, 3];
    // for element in &v {
    //     println!("element is {}", element);
    // }

    // for element in v.iter() {
    //     println!("The value is {}", element);
    // }


    // let mut v = vec![1, 2, 3];
    // for element in &mut v {
    //     // *element += 10;
    //     element = element + 20;
    // }
    //
    // let first = &v[1];
    // println!("{first}");

    // #[derive(Debug)]
    enum S {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let v =  vec![
        S::Int(3),
        S::Float(3.14),
        S::Text(String::from("hello world")),
    ];

    let value = &v[0];

    // println!("{:?}", value);
    match value {
        S::Int(i) => println!("{}",i),
        S::Float(f) => println!("{}",f),
        S::Text(s) => println!("{}",s),
    }
}
