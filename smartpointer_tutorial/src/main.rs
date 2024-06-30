use List::{Cons, Nil};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

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

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(List2::Cons(Rc::clone(&value), Rc::new(List2::Nil)));
    
    let b = List2::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = List2::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    b.print_list2();

    let a = Rc::new(List3::Cons(5, RefCell::new(Rc::new(List3::Nil))));

    // aの最初の参照カウント = {}
    println!("a initial rc count = {}", Rc::strong_count(&a));
    // aの次の要素は = {:?}
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List3::Cons(10, RefCell::new(Rc::clone(&a))));

    b.print_i32();
    // b作成後のaの参照カウント = {}
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // bの最初の参照カウント = {}
    println!("b initial rc count = {}", Rc::strong_count(&b));
    // bの次の要素 = {:?}
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // aを変更後のbの参照カウント = {}
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // aを変更後のaの参照カウント = {}
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // 次の行のコメントを外して循環していると確認してください; スタックオーバーフローします
    // println!("a  next item = {:?}", a.tail());        // aの次の要素 = {:?}

    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("{:?}, {:?}", leaf.value, leaf.children);

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node{
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}

enum List {
    Cons(i32, Rc<List>),
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

#[derive(Debug)]
enum List2 {
    Cons(Rc<RefCell<i32>>, Rc<List2>),
    Nil,
}

impl List2{
    fn print_list2(&self) {
        match self {
            List2::Cons(head, tail) => {
                println!("{:?}", head);
                println!("{:?}", tail);
                tail.print_list2();
            }
            List2::Nil => {
                println!("End of list");
            }
        }
    }
}

#[derive(Debug)]
enum List3 {
    Cons(i32, RefCell<Rc<List3>>),
    Nil,
}

impl List3{
    fn tail(&self) -> Option<&RefCell<Rc<List3>>>{
        match *self{
            List3::Cons(_, ref item) => Some(item),
            List3::Nil => None,
        }
    }

    fn print_i32(&self){
        match self{
            List3::Cons(head, _) => {
                println!("{:?}", head);
            }
            List3::Nil => {
                println!("End of list");
            }
        }
    }
}

#[derive(Debug)]
struct Node{
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
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
