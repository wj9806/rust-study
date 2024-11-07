
fn main() {
    //字符串字面值是不可变的
    let _s1 = "hello";
    //创建String类型

    let mut s2 = a();
    println!("{}, world!", s2);

    takes_ownership(s2);
    //s2被移动到函数中，所以下面一句会报错
    //println!("{}, world!", s2);

    let x = 5;
    makes_copy(x);
    //x是copy到函数中，后面可以继续使用
    println!("{}", x);

    s2 = takes_and_gives_back(a());
    println!("{}, world!", s2);
}

fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn a() -> String {
    let s2 = String::from("hello");
    //转移所有权
    let s3 = s2;
    s3
}