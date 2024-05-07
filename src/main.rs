use std::{cell::RefCell, rc::Rc};
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum List2 {
    Cons(Rc<RefCell<i32>>, Rc<List2>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(10, Rc::new(Nil)));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));

    let a2 = Rc::new(List2::Cons(Rc::clone(&value), Rc::new(List2::Nil)));
    let b2 = List2::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a2));
    let c2 = List2::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a2));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a2);
    println!("b after = {:?}", b2);
    println!("c after = {:?}", c2);
}
