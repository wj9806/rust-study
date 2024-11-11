
/// array长度固定
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {:?}", a[0]);
    println!("a = {:?}", &a[1..3]);

    //初始化一个某个值重复出现 N 次的数组, a 数组包含 5 个元素，这些元素的初始化值为 3
    let _a = [3; 5];

    //定义二维数组
    let b = [a, _a];

    let array: [String; 4] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:#?}", array);

    //数组切片
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
