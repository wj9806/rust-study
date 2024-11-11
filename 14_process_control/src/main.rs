
//if 语句块是表达式，这里我们使用 if 表达式的返回值来给 变量 进行赋值
fn main() {
    if_func();
    for_func();
    while_func();
    loop_func();
}

///  break 可以单独使用，也可以带一个返回值，有些类似 return
/// loop 是一个表达式，因此可以返回一个值
fn loop_func() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_func() {
    let mut x = 0;
    while x < 5 {
        x = x + 1;
        println!("{}", x);
    }

}

/// for 元素 in 集合 {
///
/// }
fn for_func() {
    for i in 1..5 {
        println!("{}", i);
    }

    /**
    *
        如果想在循环中，修改该元素，可以使用 mut 关键字：
        for item in &mut collection {
          // ...
        }
    */
    //如果想在循环中获取元素的索引：
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        if i == 2 {
            break;
        }
        println!("第{}个元素是{}", i + 1, v);
    }
}

fn if_func() {
    let a = 1;
    let b = if a > 1 {
        1
    } else if a == 1 {
        2
    } else {
        3
    };
    println!("{}", b);
}
