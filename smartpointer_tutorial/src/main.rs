use List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    
    let list = Cons(1,
                Box::new(Cons(2,
                    Box::new(Cons(3,
                        Box::new(Nil))))));

    print_list(&list);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

enum List {
    Cons( i32, Box<List> ),
    Nil,
}

fn print_list(list: &List) {
    match list {
        Cons(head, tail) => {
            println!("{}", head);
            print_list(tail);
        }
        Nil => {
            println!("End of list");
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}

fn hello(name: &str){
    println!("Hello {}!", name);
}
