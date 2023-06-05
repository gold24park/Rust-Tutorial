use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    // iter: 컬렉션의 immutable reference를 생성하여 iteration 수행
    // 컬렉션의 요소를 읽기만 가능하고 수정할 수 없는 상대로 iteration함. 이를통해 컬렉션 요소에 안전하게 접근

    // into_iter: 컬렉션의 소유권을 이전하여 소유권 이동을 수행. 컬렉션의 요소들을 소비하며,
    // 컬렉션은 반복이 완료되면 비어있는 상태가 된다. 이를 통해 컬렉션을 수정하거나 소비하는 작업을 할 수 있다.
    assert_eq!(v1.iter().next(), Some(&1));
    println!("{:?}", v1);
    assert_eq!(v1.into_iter().next(), Some(1));
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

fn execute_closure<F>(closure: F)
where
    F: Fn(i32) -> i32,
{
    let result = closure(10);
    println!("Result: {}", result);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

fn main() {
    let add_one = |x| x + 1;
    execute_closure(add_one);

    let v1 = vec![1, 2, 3];
    let a: Vec<_> = v1.iter().map(|x| x + 1).collect();
}
