
fn main() {
    let _c = 'z';
    let _z = 'ℤ';
    let _g = '国';
    let _heart_eyed_cat = '😻';

    println!("字符'😻'占用了{}字节的内存大小",std::mem::size_of_val(&_heart_eyed_cat));

    let t = true;
    println!("bool占用了{}字节的内存大小",std::mem::size_of_val(&t))
}