use std::ops::Deref;
use List::{Cons, Nil};

/// A representation of a cons list.
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// A custom implementation of `Box` that is used to understand the `Deref` trait.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(data: T) -> MyBox<T> {
        MyBox(data)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn run() {
    // Basic example of a Box
    // let b = Box::new(5);
    // println!("b = {}", b);

    // In order for the compiler to determine the size of a `List`, we must specificy the second
    // argument of `Cons` to be a `Box<List>` (i.e., a pointer to `List`). Otherwise, the size
    // is infinite since the type would be recursive.
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Usage of `MyBox<T>`
    // let x = 5;
    // let y = MyBox::new(5);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
