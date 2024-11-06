use num::complex::Complex;

fn main() {
    //rust 类型推导
    //给 guess 变量一个显式的类型标注：let guess: i32 = ... 或者 "42".parse::<i32>()。
    let _guess = "42".parse::<i32>().expect("Not a number!");

    //使用 wrapping_* 方法在所有模式下都按照补码循环溢出规则处理，例如 wrapping_add
    //如果使用 checked_* 方法时发生溢出，则返回 None 值
    //使用 overflowing_* 方法返回该值和一个指示是否存在溢出的布尔值
    //使用 saturating_* 方法，可以限定计算后的结果不超过目标类型的最大值或低于最小值，例如:
    let a : u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);  // 19

    let b = a.saturating_add(20);
    println!("{}", b);  // 19

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // 断言0.1 + 0.2与0.3不相等
    assert_ne!(0.1 + 0.2, 0.3);

    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("NaN");
    }

    //定义一个f32数组，其中42.0会自动被推导为f32类型
    let _forty_twos = [
        42.0,
        42.0,
        42.0_f32,
    ];

    //序列
    for i in 1..=5 {
        println!("{}", i);
    }

    let a = Complex { re: 1.0, im: -1.0 };
    let b = Complex::new(11.1, 22.2);
    let res = a + b;

    println!("{} + {}i", res.re, res.im)
}