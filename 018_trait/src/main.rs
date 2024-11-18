use std::fmt::{Debug, Display};

//定义特征
pub trait Summary {
    //可以在特征中定义默认实现
    fn summarize(&self) -> String {
        format!("(Read more...)")
    }
}

pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

//实现特征
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.title, self.author, self.content)
    }
}

pub struct WeiBo {
    pub title: String,
    pub author: String,
}

impl Summary for WeiBo {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}

//先定义一个函数，使用特征作为函数参数
fn notify(s: &impl Summary) {
    println!("notify: {}", s.summarize());
}
//先定义一个函数，使用特征作为函数参数
fn notify2<T: Summary>(s: &T) {
    println!("notify: {}", s.summarize());
}
//多重约束
pub fn notify3(item: &(impl Summary + Display)) {

}
pub fn notify4<T: Summary + Display>(item: &T) {

}
//where约束
pub fn notify5<T: Summary + Display, U: Clone + Debug>(item: &T, item2: &U) {

}
pub fn notify6<T, U>(item: &T, item2: &U) where T: Summary + Display, U: Clone + Debug {

}

//可以通过 impl Trait 来说明一个函数返回了一个类型，该类型实现了某个特征
fn returns_summarizable() -> impl Summary {
    WeiBo {
        title: String::from("sunface"),
        author: String::from(
            "weibo",
        )
    }
}

//特征定义了一组可以被共享的行为，只要实现了特征，你就能使用这组行为。
fn main() {
    let post = Post {title: String::from("title"), author: String::from("author"), content: String::from("content") };
    let weibo = WeiBo {title: String::from("title"), author: String::from("author") };
    println!("We have {} posts!", post.summarize());
    println!("{}", weibo.summarize());
}