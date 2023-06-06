mod cons_list;
mod drop;
mod my_box;

use crate::cons_list::List::{Cons, Nil};
use crate::drop::CustomSmartPointer;
use crate::my_box::MyBox;
use std::mem::drop;

#[test]
fn box_ref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let m = MyBox::new(String::from("Rust"));
    // [?] deref coercions 왜? MyBox가 Deref Trait을 따르지 않았다면 hello(&(*m)[..]); 이렇게 써야했을것
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
