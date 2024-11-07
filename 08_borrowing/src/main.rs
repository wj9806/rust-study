
fn main() {
    let x = 5;
    let y = &x;
    let z = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let mut s1 = String::from("hello");
    //& 符号即是引用，它们允许你使用值，但是不获取所有权
    //引用指向的值默认也是不可变的
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    //创建一个可变的引用 &mut s1
    change_string(&mut s1);
    println!("{}", s1);

    //不可变引用可以借用多次
    // 同一作用域，特定数据只能有一个可变引用：
    //可变引用与不可变引用不能同时存在
    let mut s = String::from("hello");
    let r1 = &mut s;
    //let r2 = &mut s;
    println!("{}", r1);

    test();
}

//接受可变引用参数 &mut String
fn change_string(s: &mut String) {
    s.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn test() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    //新编译器中 引用作用域的结束位置从花括号变成最后一次使用的位置
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
}// 老编译器中，r1、r2、r3作用域在这里结束
// 新编译器中，r3作用域在这里结束