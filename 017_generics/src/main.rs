use std::fmt::Display;

fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

fn create_and_print<T>() where T: From<i32> + Display {
    let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
    //println!("a is: {}", a);
}

//结构体中使用泛型
//x 和 y 是相同的类型
struct Point<T> {
    x: T,
    y: T,
}

//方法中使用泛型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//对于 Point<T> 类型，你不仅能定义基于 T 的方法，还能针对特定的具体类型，进行方法定义
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//将 const fn 与 const 泛型 结合
struct Buffer<const N: usize> {
    data: [u8; N],
}

const fn compute_buffer_size(factor: usize) -> usize {
    factor * 1024
}


fn main() {
    println!("{}", add(1, 2));
    create_and_print::<i64>();
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    println!("p.x = {}", _float.x());

    const SIZE: usize = compute_buffer_size(4);
    let buffer = Buffer::<SIZE> {
        data: [0; SIZE],
    };
    println!("Buffer size: {} bytes", buffer.data.len());
}