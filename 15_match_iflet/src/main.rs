use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    let mut dir = Direction::East;
    match_dir(dir);
    dir = Direction::South;
    match_dir(dir);

    //
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            //从模式中取出绑定的值
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                         r, g,
                );
            }
        }
    }

    if_let();

    let v = vec![Direction::South, Direction::West, Direction::East, Direction::South];
    let v2: Vec<&Direction> = v.iter().filter(|x| matches!(x, Direction::South)).collect();
    for x in v2 {
        println!("{:#?}", x)
    }

    //无论是 match 还是 if let，这里都是一个新的代码块，而且这里的绑定相当于新变量，如果你使用同名变量，会发生变量遮蔽：
    let age = Some(30);
    println!("在匹配前，age是{:?}",age);
    if let Some(age) = age {
        println!("匹配出来的age是{}",age);
    }
    println!("在匹配后，age是{:?}",age);

    println!("{:?}", add(None));
    println!("{:?}", add(Some(2)));

    while_let();

    let_else();

    //..= 语法允许你匹配一个闭区间序列内的值
    let a = 5;
    match a {
        1..=9 => println!("1-9"),
        _ => println!("anther")
    }

    match_struct();

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    match_guard();

    at_binding();
}
enum Message {
    Hello { id: i32 },
}
fn at_binding() {
    let msg = Message::Hello { id: 5 };

    match msg {
        //我们希望测试 Message::Hello 的 id 字段是否位于 3..=7 范围内，
        // 同时也希望能将其值绑定到 id_variable 变量中以便此分支中相关的代码可以使用它。
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }

    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }

    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point {x: 10, y: 5};
    if let p @ Point {x: 10, y} = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}

fn match_guard() {
    let num = Some(4);

    match num {
        //匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件，它能为分支模式提供更进一步的匹配条件。
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}


fn match_struct() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 7 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn let_else() {

    //使用 let-else 匹配，即可使 let 变为可驳模式。它可以使用 else 分支来处理模式不匹配的情况，
    // 但是 else 分支中必须用发散的代码块处理（例如：break、return、panic）
    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' ');
        let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
            panic!("Can't segment count item pair: '{s}'");
        };
        let Ok(count) = u64::from_str(count_str) else {
            panic!("Can't parse integer: '{count_str}'");
        };
        // error: `else` clause of `let...else` does not diverge
        // let Ok(count) = u64::from_str(count_str) else { 0 };
        (count, item)
    }

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}

fn while_let() {
    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("stack: {}", top);
    }
}

//解构Option
fn add(s: Option<i32>) -> Option<i32> {
    match s {
        None => None,
        Some(i) => Some(i + 1),
    }
}

//当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。
fn if_let() {
    let s = Some(1);
    //if let也支持模式绑定
    if let Some(i) = s {
        println!("{}", i);
    }
    if let Some(1) = s {
        println!("match Some(1)");
    }
}

///  match 跟其他语言中的 switch 非常像，_ 类似于 switch 中的 default。
/// match 本身也是一个表达式，因此可以用它来赋值
fn match_dir(dir: Direction) {
    match dir {
        Direction::East => println!("East"),
        Direction::West | Direction::North => println!("West or North"),
        //() 表示返回单元类型与所有分支返回值的类型相同，所以当匹配到 _ 后，什么也不会发生。
        _ => ()
    };
}
