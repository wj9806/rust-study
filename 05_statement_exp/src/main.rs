
fn main() {

    let _a = 8;
    let _b: Vec<f64> = Vec::new();
    let (_a, _c) = ("hi", false);

    let y = {
        let x = 3;
        //表达式不能包含分号
        x + 1
    };

    println!("The value of y is: {}", y);

    assert_eq!(ret_unit_type(), ())
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let _y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    // 或者写成一行
    let _z = if x % 2 == 1 { "odd" } else { "even" };

    //表达式如果不返回任何值，会隐式地返回一个 ()
}