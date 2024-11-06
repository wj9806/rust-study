//const 标注的是常量
const PI: f32 = 3.14;

///Rust 的变量在默认情况下是不可变的。
fn main() {
    //mut 可变的变量
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //使用下划线开头忽略未使用的变量
    let _y = 10;

    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);

    println!("{}", PI);

    //变量遮蔽
    //Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;
    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);


    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}