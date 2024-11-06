
fn main() {
    println!("{}", add(1, 1));

    println!("{}", plus_or_minus(10));

    forever();
    let a = 10;
}

fn plus_or_minus(x:i32) -> i32 {
    if x > 5 {
        return x - 5
    }
    x + 5
}

//当用 ! 作函数返回类型的时候，表示该函数永不返回( diverge function )，特别的，这种语法往往用做会导致程序崩溃的函数
fn forever() -> ! {
    loop {
        //...
    };
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}