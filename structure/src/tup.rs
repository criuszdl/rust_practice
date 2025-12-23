fn main() {
    //元组类型
    let tup = (500,true, "Rust");
    println!("tup 0  is {}.", tup.0);
    println!("tup 1  is {}.", tup.1);
    println!("tup 2  is {}.", tup.2);

    let (x,y,z) = tup;
    println!("x is {}. y is {}. z is {}", x, y, z);

    //显式声明类型
    let tup2:(i32,f64,char) = (32,6.4,'a');
    let (x,y,z) = tup2;
    println!("x is {}. y is {}. z is {}", x,y,z);
}
