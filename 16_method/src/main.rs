
//当从模块外部访问结构体时，结构体的字段默认是私有的，其目的是隐藏信息（封装）
mod my {
    pub struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    //Rust 允许我们为一个结构体定义多个 impl 块
    impl Circle {
        //定义在 impl 中且没有 self 的函数被称之为关联函数
        //因此它是一个函数而不是方法
        //因为是函数，所以不能用 . 的方式来调用，我们需要用 :: 来调用
        pub fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x,
                y,
                radius
            }
        }
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }

        pub fn equals(&self, other: &Circle) -> bool {
            other.x == self.x && other.y == self.y && other.radius == self.radius
        }
    }
}

fn main() {
    let c = my::Circle::new(0.0, 0.0, 2.0);
    println!("{}", c.area());
    let c2 = my::Circle::new(0.0, 0.0, 2.0);
    let c3 = my::Circle::new(0.0, 0.0, 3.0);
    println!("{}", c.equals(&c2));
    println!("{}", c.equals(&c3));
}