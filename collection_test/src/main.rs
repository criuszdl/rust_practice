// use std::collections::HashMap;
//
// fn main() {
//     //给定一组整数，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（出现次数最多的值；在这里哈希 map 会很有帮助）。
//
//     // ---------- 中位数 ----------
//     let mut nums = vec![1,2,5,4,5];
//
//     nums.sort();
//
//     let middle_index = nums.len() / 2;
//     assert_eq!(middle_index, 2);
//
//     let middle = nums[middle_index];
//
//     // ---------- 众数 ----------
//
//     let mut map = HashMap::new();
//     for element in &nums {
//         let count = map.entry(*element).or_insert(0);
//         *count += 1;
//     }
//     println!("{:?}", map);
//
//     let mut model = nums[0];
//     let mut max_count = 0;
//     for (key, value) in map {
//         if value > max_count {
//             max_count = value;
//             model = key;
//         }
//     }
//
//     println!("排序后的中位数是：{:?}", middle);
//     println!("众数是：{:?}", model);
//     println!("众数出现次数是：{:?}", max_count);
// }

//将字符串转换为 pig latin。
// 也就是每一个单词的第一个辅音字母被移动到单词的结尾并增 加 ay，所以 first 会变成 irst-fay。
// 元音字母开头的单词则在结尾增加 hay（apple 会变成 apple-hay）。请注意 UTF-8 编码的细节！

// fn main() {
//     let sentence = "hello  apple 中文 rust";
//     let result = sentence
//         .split_whitespace()
//         .map(|word| to_pig_latin(word))
//         .collect::<Vec<_>>()
//         .join(" ");
//     println!("{:?}", result);
// }
//
// fn to_pig_latin(word: &str) -> String {
//     let mut chars = word.chars();
//     let first = chars.next().unwrap();
//     let vowels = ['a', 'e', 'i', 'o', 'u'];
//     if vowels.contains(&first.to_ascii_lowercase()) {
//         format!("{}-hay", word)
//     }else {
//         let rest:String = chars.collect();
//         format!("{}-{}ay", rest, first)
//     }
// }

//第三道题：
use std::collections::HashMap;
struct Company {
    //什么部门，对应哪些员工
    //{
    //   "Engineering": ["Sally", "Bob"],
    //   "Sales": ["Amir"]
    // }
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    fn add_employee(&mut self,name: String, dept: String){
        let employees = self.departments.entry(dept).or_insert(Vec::new());
        employees.push(name);
    }

    fn list_department(&self, dept: &str){
        if let Some(employees) = self.departments.get(dept) {
            let mut list = employees.clone();
            list.sort();
            println!("{}部门的员工是{list:#?}",dept);
        }else {
            println!("{} 部门不存在", dept);
        }
    }

    fn list_all(&self) {
        let mut depts:Vec<_> =  self.departments.iter().collect();
        depts.sort_by_key(|(dept, _)| *dept);
        for(dept , employees) in depts {
            let mut list = employees.clone();
            list.sort();
            println!("{}:{:#?}",dept,list);
        }
    }
}

fn main() {
    let mut company = Company::new();

    // 模拟输入
    company.add_employee("Sally".to_string(), "Engineering".to_string());
    company.add_employee("Bob".to_string(), "Engineering".to_string());
    company.add_employee("Amir".to_string(), "Sales".to_string());

    // company.list_department("Engineering");
    company.list_all();
}
