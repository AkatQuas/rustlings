// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `unimplemented!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` for hints :)

use std::ops::Deref;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// should compile correctly
#[warn(non_snake_case)]
fn test_MyBox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// should compile correctly
#[warn(non_snake_case)]
fn test_RcList() {
    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    println!("Count after creating a. [count = {}]", Rc::strong_count(&a));

    let _b = RcList::Cons(3, Rc::clone(&a));
    println!("Count after creating b. [count = {}]", Rc::strong_count(&a));

    {
        let _c = RcList::Cons(4, Rc::clone(&a));
        println!("Count after creating c. [count = {}]", Rc::strong_count(&a));
    }
    println!(
        "Count after c goes out of scope. [count = {}]",
        Rc::strong_count(&a)
    );
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(42, create_empty_list())
    );
    test_MyBox();
    test_RcList();
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list(val: i32, exist: List) -> List {
    List::Cons(val, Box::new(exist))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(
            create_empty_list(),
            create_non_empty_list(5, create_empty_list())
        )
    }
}
