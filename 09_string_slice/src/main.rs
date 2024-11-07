use std::ops::Add;

fn main() {
    slice();
    convert();
    push();
    replace();
    del();
    concat();
    utf8();
}

fn utf8() {
    for c in "中国人".chars() {
        println!("{}", c);
    }

    for b in "中国人".bytes() {
        println!("{}", b);
    }
}

fn concat() {
    //1、使用 + 或者 += 连接字符串 要求右边的参数必须为字符串的切片引用（Slice）类型
    let mut s = String::from("hello");
    s += " world";
    println!("{}", s);
    //调用add后，s的所有权被转移了 所有权需要重新转移到s
    s = s.add("!");
    println!("{}", s);
}

fn convert() {
    let /*mut*/ s = "hello world".to_string();
    let word = first_word(&s);
    //s.clear(); // error!
    println!("the first word is: {}", word);
    println!("{}", s);
    //String转&str
    println!("{}", &s);
    println!("{}", &s[..]);
    println!("{}", s.as_str());
}

fn slice() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..11];
    let slice = &s[..];
    println!("{} {}", hello, world);
    println!("{}", slice);
}

fn push() {
    let mut s = String::from("Hello ");
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);
    s.push('!');
    println!("追加字符 push() -> {}", s);
    s.insert_str(6, " I like ");
    println!("插入字符串 insert_str() -> {}", s);
}

fn replace() {
    //替换字符串
    //1.replace 该方法是返回一个新的字符串，而不是操作原来的字符串。
    //2.replacen 该方法是返回一个新的字符串，而不是操作原来的字符串。
    //3.replace_range 该方法是直接操作原来的字符串，不会返回新的字符串。该方法需要使用 mut 关键字修饰。
    let mut s = "hello world".to_string();
    s.replace_range(0..1, "H");
    println!("{}", s);
}

fn del() {
    //删除字符串
    //1.pop —— 直接操作原来的字符串，删除并返回字符串的最后一个字符,其返回值是一个 Option 类型
    //2.remove —— 删除并返回字符串中指定位置的字符 只接收一个参数
    //3.truncate —— 删除字符串中从指定位置开始到结尾的全部字符 无返回值
    //4.clear —— 清空字符串 相当于 truncate() 方法参数为 0 的时候。
    let mut s = String::from("hello world.");
    let c = s.pop();
    println!("{}", c.unwrap());
    println!("{}", s);

    s.truncate(5);
    println!("{}", s);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}