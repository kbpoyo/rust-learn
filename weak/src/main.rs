use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive( Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));

   println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
   println!("a指向的节点 = {:?}", a.tail());

   let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
   println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
   println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
   println!("b指向的节点 = {:?}", b.tail());

   if let Some(link) = a.tail() {
    *(link.borrow_mut()) = Rc::clone(&b);
   }

   println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
   println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

   // 下面一行println!将导致循环引用
    // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
    println!("a next item = {:?}", a.tail());

}

#[test]
fn test() {
    let five = Rc::new(5);

    let weak_five = Rc::downgrade(&five);

    println!("five strong_count = {}", Rc::strong_count(&five));
    
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("five strong_count = {}", Rc::strong_count(&five));
    assert_eq!(*(strong_five.unwrap()), 5);
    println!("five strong_count = {}", Rc::strong_count(&five));

    drop(five);

    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    assert_eq!(strong_five, None);
}
