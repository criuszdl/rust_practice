#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    pages: u32,
    available: bool
}

impl Book {
    //关联函数 new：创建一个新书实例（所有字段都要传入）。
    fn new(title: String,author: String, isbn: String,pages: u32) -> Book {
        Book {
            title,
            author,
            isbn,
            pages,
            available:true //默认可借出
        }
    }
    //方法 borrow_book：借书。如果书可用（available == true），将 available 改为 false 并返回 true；否则返回 false。
    //借用之 - 不可变引用与可变引用
    fn borrow_book(&mut self) -> bool {
        if self.available {
            self.available = false;
            true
        }else {
            false
        }
    }
    //方法 return_book：还书。将 available 改为 true。
    fn return_book(&mut self) {
        self.available = true;
    }
    //方法 info：返回一个字符串，展示书的详细信息（比如："《Rust编程》 by Ferris, ISBN: 978-1234567890, 500页, 状态: 可借"）。
    fn info(&self)  -> String {
        // 报错，错误
        // self.title.as_str() + String::from(" by ") +
        //     self.author.as_str() + String::from(" ISBN: ") +
        //     self.isbn.as_str() +String::from(" , ") +
        //     self.pages.to_string().as_str() + String::from(" 状态: ") +
        //     if self.available {String::from(" 可借 ")} else {String::from("不可借")}
        let status = if self.available {"可借"} else { "已借出" };
        format!(
            "《{}》 by {}, ISBN: {}, {}页, 状态: {}",
            self.title,self.author,self.isbn,self.pages,status
        );
    }
}

fn main(){
    let mut book1 = Book::new(
        "Rust程序设计语言".to_string(),
        "Steve Niklaas & Carol Nichols".to_string(),
        "978-7115544476".to_string(),
        600,
    );

    let book2 = Book::new(
        "深入浅出Rust".to_string(),
        "张三".to_string(),
        "978-1234567890".to_string(),
        400,
    );

    let book3 = Book::new(
        "The Little Book of Rust".to_string(),
        "Unknown".to_string(),
        "978-0000000000".to_string(),
        200,
    );

    //打印每本书的 info。
    println!("{}", book1.info());
    println!("{}", book2.info());
    println!("{}", book3.info());

    println!("\n--- 尝试借书 ---");
    if book1.borrow_book() {
        println!("{}", "借书成功！");
    }else{
        println!("{}", "借书失败，已被借出");
    }

    println!("{}", book1.info());

    //再次借同一本书
    if book1.borrow_book() {
        println!("{}", "借书成功！");
    }else{
        println!("{}", "借书失败，已被借出");
    }

    println!("\n--- 还书 ---");
    book1.return_book();
    if book1.borrow_book() {
        println!("{}", "再次借书成功！");
    }else{
        println!("{}", "再次借书成功失败，已被借出");
    }
}