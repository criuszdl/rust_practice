fn main() {
    //初始化空的字符串
    // let mut s =  String::new();

    //初始化有值的
    // let data = "initial contents";
    // let s = data.to_string();
    //
    // let s2 = "initial contents2".to_string();
    //
    // let s3 = String::from("initial contents3");
    //
    // println!("{s}");
    // println!("{s2}");
    // println!("{s3}");

    //使用 push_str 和 push 附加字符串
    // let mut sp = String::from("hello");
    // sp.push_str(", world!");
    // println!("{}", sp);

    // let mut s1 = String::from("hello");
    // let s2 = "world";
    // s1.push_str(s2);
    // println!("{}", s2);

    // let mut s = String::from("lo");
    // s.push('l');
    // println!("{}",s)

    //使用 + 运算符或 format! 宏拼接字符串
    // let s1 = String::from("lo");
    // let s2 = String::from("12");
    // let s3 = s1 + &s2;
    // println!("{}", s3);
    // println!("{}", s2);
    // println!("{}", s1);//错误，已被借用

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // // let s = s1 + "-" + &s2 + "-" + &s3;
    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("{}", s);
    // println!("{}", s1);

    //索引字符串
    // let s1 = String::from("hi");
    // let h = s1[0];

    // let hello = "Здравствуйте";
    // let s = &hello[0..4];

    for e in "Зд".chars() {
        println!("{}", e);
    }

    for e in "Зд".bytes() {
        println!("{}", e);
    }












}
