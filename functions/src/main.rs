fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    // statement: instructions that perform some action and do not return a value
    let y = 6; // let은 statement: 왜냐면 let x = (let y = 6) 같은게 성립 안됨

    // expression: evaluate to a resultant value
    let z = {
        let x = 3;
        x + 1
    }; // {}는 expression. evaluates to 4.
    // expression의 마지막에 세미콜론을 붙이면 statement가 되어버림.
    println!("z: {z}");

    let k = five();
    println!("k: {k}");

    let j = plus_one(5);
    println!("j: {j}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}