/*mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    //相对路径
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}*/

/*mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} as toast please", meal.toast);

    // meal.seasonal_fruit  = String:: from("blueBerries");
}*/

/*mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {

    //第一种方法 + 配合 hosting::add_to_waitlist();
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        //绝对路径
        // crate::front_of_house::hosting::add_to_waitlist();
        //相对路径
        // front_of_house::hosting::add_to_waitlist();
        //使用use关键字
        // hosting::add_to_waitlist();
        //第二种方法
        super::hosting::add_to_waitlist();
    }
}*/

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1,2);
    println!("{:?}", map);
}
