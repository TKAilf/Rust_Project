use List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let b = Rc::new(5);
    println!("b = {}", b);
    
    let list = Cons(1,
                Rc::new(Cons(2,
                    Rc::new(Cons(3,
                        Rc::new(Nil))))));

    print_list(&list);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Rc::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let _c = CustomSmartPointer{data: String::from("my stuff")};
    let _d = CustomSmartPointer{data: String::from("other stuff")};
    println!("CustomSmartPointers created.");

    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating a = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating a = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

enum List {
    Cons( i32, Rc<List> ),
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

struct CustomSmartPointer{
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}
