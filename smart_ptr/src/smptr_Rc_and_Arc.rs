use std::rc::Rc;
use std::sync::Arc;
use std::thread;

#[test]
fn test() {
    let a = Rc::new(String::from("hello, rust"));

    println!("a count = {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);

    println!("a count = {}", Rc::strong_count(&a));
    println!("b count = {}", Rc::strong_count(&b));

    {
        let c = Rc::clone(&b);
        println!("c count = {}", Rc::strong_count(&c));
    }


    println!("a count = {}", Rc::strong_count(&a));

}

#[test]
fn test_1() {
    let s = Arc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {println!("{}", s)});
    }
}





