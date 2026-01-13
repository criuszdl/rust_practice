use std::collections::HashMap;

fn main() {

    // let mut map = HashMap::new();
    //
    //  map.insert(String::from("yellow"),1);
    //  map.insert(String::from("pink"),2);

    //===================  -  ========================

    // let team_name = String::from("pink");
    // let score = map.get(&team_name).copied().unwrap_or(0);

    // println!("{}", score);

    // for (key ,value) in &map {
    //     println!("{} : {}", key,value);
    // }

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    //
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // println!("{}",field_name);

    // map.insert(String::from("pink"),9);
    // for (key ,value) in &map {
    //     println!("{} : {}", key,value);
    // }
    // println!("{:#?}", map);
    // println!("{:?}", map);

    // let mut h = HashMap::new();
    // h.insert(String::from("pink"),9);
    //
    // h.entry(String::from("pink")).or_insert(90);
    // h.entry(String::from("yellow")).or_insert(100);
    //
    // println!("{h:#?}")


    // let text = "hello world wonderful world";
    //
    // let mut map = HashMap::new();
    //
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    // println!("{:#?}", &map);
    //
    // match map.get("hello") {
    //     Some(val) => println!("{:#?}", val),
    //     None => println!("not found"),
    // }
    //
    // if let Some(val) = map.get("world") {
    //     println!("{:#?}", val);
    // }


}
