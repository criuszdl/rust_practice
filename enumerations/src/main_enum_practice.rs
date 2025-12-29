//一个不会产生panic 的简单的数字除法
fn checked_division(dividend: i32 , divisor: i32) -> Option<i32> {
    if divisor == 0 {
        //失败返回None变体
        None
    }else{
        //返回结果是一个包装的Some变体
        Some(dividend / divisor)
    }
}

//这个函数执行一个可能不会成功的除法
fn try_division(dividend: i32 , divisor: i32){
    //‘ Option ’值可以进行模式匹配，就像其他枚举一样
    match checked_division(dividend, divisor) {
        None =>println!(" {} / {} failed", dividend, divisor),
        Some(quotient) =>println!(" {} / {} = {}", dividend, divisor, quotient)
    }
}

fn main() {
    // try_division(4,2);
    // try_division(1,0);

    //将‘ None ’绑定到变量需要进行类型注释
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    //展开‘ Some ’变体将提取所包装的值。
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());
    //展开‘ None ’变体将panic。
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}

