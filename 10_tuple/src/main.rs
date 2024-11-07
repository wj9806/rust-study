
fn main() {
    //定义元组
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    //可以使用.来访问元组
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{} {} {}", five_hundred, six_point_four, one);

    let tup = ret_tuple();
    println!("{} {} {}", tup.0, tup.1, tup.2);
}

//函数返回元组
fn ret_tuple() -> (i32, f64, u8) {
    (1, 2.0, 1)
}