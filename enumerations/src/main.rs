
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         //Coin::Penny => 1,
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         },
//     }
// }
//
// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
//     value_in_cents(Coin::Penny);
// }

// fn plus_one(x : Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(x) => {Some(x + 1)},
//     }
// }

fn main() {
    // let five = Some(5);
    // let six = plus_one(five);
    // println!("current value is {:?}",six);
    // let none = plus_one(None);
    // println!("current value is {:?}",none);

    // let dice_roll = 9;
    // match dice_roll {
    //     1 => println!("laugh"),
    //     9 => println!("cry"),
    //     _ => (),
    // }

    // let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }

    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {max}")
    // }

    // let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => count += 1,
    // }

    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {state:?}!")
    // }else {
    //     count += 1;
    // }
}

enum UsState {
    Alabama,
    Alaska,
}

//检查州的“年龄”
impl UsState {
    fn existed_in (&self,year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            //其他条件等等
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

//基础版：使用 if let 来匹配硬币的类型，在条件代码中引入一个 state
fn describe_state_quarter(coin : Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in (1990) {
            Some(format!("Quarter has a leap year!"))
        }
    }else {
        None
    }
}

//进阶版1：可以利用这个表达式要么从 if let 中生成一个 state 要么提前返回的优势
fn describe_state_quarter(coin : Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    }else {
        None
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    }else {
        Some(format!("{state:?} is relatively new."))
    }
}

//进阶版2：如果模式匹配，它会将匹配到的值绑定到外层作用域。如果模式不匹配，程序流会指向 else 分支，它必须从函数返回。
fn describe_state_quarter(coin : Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        None
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    }else {
        Some(format!("{state:?} is relatively new."))
    }
}
//以这种方式在函数主体中保持了“愉快路径”（“Happy Path”），而不用像 if let 那样在两个分支中拥有明显不同的控制流.
