use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[test]
fn test() {
    let c = Cell::new("asdf");
    let one = c.get();

    c.set("qwer");
    let two = c.get();

    println!("{}, {}", one, two);
}

#[test]
fn test_1() {
    // code snipet 1
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());

// code snipet 2
    // let mut x = 1;
    // let y = &mut x;
    // let z = &mut x;
    // x = 2;
    // *y = 3;
    // *z = 4;
    // println!("{}", x);

}

#[test]
fn test_2() {
    let s = RefCell::new(String::from("hellow rust"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();

    println!("{}, {}", s1, s2);
}

pub trait Messenger {
    fn send (&self, msg: String);
    
}

pub struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send (&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg);
    }
}


#[test]
fn test_3() {
    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::<String>::new()),
    };

    mq.send("hello, world".to_string());
}

#[test]
fn test_4() {
    let s = Rc::new(RefCell::new("hello".to_string()));
    let s1 = Rc::clone(&s);
    let s2 = Rc::clone(&s);
    s2.borrow_mut().push_str("string");

    println!("{:?}, {:?}, {:?}", s, s1, s2);

}