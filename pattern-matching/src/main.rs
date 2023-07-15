fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
        // y scope ends
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 다중 패턴
    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 범위 매칭
    let x = 5.0;
    match x {
        1.0..=5.0 => println!("one through five"),
        _ => println!("anything else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 구조체 해체
    let p = Point { x: 4, y: 7 };
    if let Point { x: a, y: b } = p {
        println!("a + b = {:?}", a + b);
    }
    let Point { x: a, y: b } = p;
    println!("a * b = {:?}", a * b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 열거형 해체
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // 참조자 해체
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    println!("sum of squares: {}", sum_of_squares);

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("x + y + feet + inches = {}", x + y + feet + inches);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let s = Some(String::from("Hello!"));

    // underscore는 소유권을 가져가지 않음
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let numbers = (2, 4, 5, 7);

    match numbers {
        (first, second, ..) => {
            println!("Some numbers: {}, {}", first, second);
        }
    }

    // ref & ref mut
    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref mut name) => {
            *name = String::from("Lou");
        }
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);

    // 매치가드
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(50);
    let y = 10;

    match x {
        th @ Some(50) => println!("Got 50: {:?}", th),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // at binding
    let msg = AtMessage::Hello { id: 5 };
    match msg {
        AtMessage::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        AtMessage::Hello { id: id @ 10..=12 } => {
            println!("Found an id in another range: {}", id)
        }
        _ => println!("No special id found"),
    }
}

enum AtMessage {
    Hello { id: i32 },
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Point {
    x: i32,
    y: i32,
}
