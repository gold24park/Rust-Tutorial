mod cons_list;
mod drop;
mod graph;
mod my_box;

use crate::cons_list::List::{Cons, Nil};
use crate::drop::CustomSmartPointer;
use crate::graph::Node;
use crate::my_box::MyBox;
use std::cell::RefCell;
use std::mem::drop;
use std::rc::{Rc, Weak};

#[test]
fn box_ref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn node() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    dbg!(Rc::strong_count(&a));

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    dbg!(Rc::strong_count(&a));
    dbg!(Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    dbg!(Rc::strong_count(&a));
    dbg!(Rc::strong_count(&b));

    node();

    /*
    a after = Cons(RefCell { value: 15 }, Nil)
    b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
     */
}
