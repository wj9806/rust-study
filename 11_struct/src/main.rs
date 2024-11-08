
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let u = build_user("zhangsan@qq.com".to_string(), "zhangsan".to_string());
    let _u2 = User {
        email: "lisi@qq.com".to_string(),
        //.. 语法表明凡是我们没有显式声明的字段，全部从 u 中自动获取。
        ..u
    };

    println!("{}", u.active);
    //报错：username 所有权被转移给了 u2，导致了 u1 无法再被使用
    //println!("{}", u.username);

    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // dbg! 宏，它会拿走表达式的所有权  除此之外，它最终还会把表达式值的所有权返回！
    rect1 = dbg!(rect1);
    println!("rect1 is {:#?}", rect1);
}

fn build_user(email: String, username: String) -> User {
    User {
        // username: username,
        // email: email,
        username,
        email,
        sign_in_count: 0,
        active: false,
    }
}