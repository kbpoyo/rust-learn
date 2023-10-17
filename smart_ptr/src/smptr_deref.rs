use std::ops::{Deref, DerefMut};
use std::fmt::Display;
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

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
    
}

impl<T> Display for MyBox<T> where T: Display {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn display(s: &str) {
    println!("{}", s);
}

fn display_mut(s: &mut str) {
    println!("{}", s);
}

#[test]
fn test_4() {
    let m = MyBox::<String>::new(String::from("hello rust"));
    println!("{}", m);
}

#[test]
fn test_3() {
    let m = MyBox::<String>::new(String::from("Rust"));
    display(&m);

    let s = m.to_string();
    println!("s = {}", s);

    let mut n = MyBox::<String>::new(String::from("CPP"));
    display_mut(&mut n);

    let mut s = n.to_string();
    println!("s = {}", s);

}

#[test]
fn test_2() {
    let y = MyBox::<u32>::new(5);

    println!("*y = {}", *y);

    
}

#[test]
fn test_1() {
    let x = 5;
    let share_ref = &x;
    println!("&x = {:p}", share_ref);
    println!("&&x = {:p}", &share_ref);

    let mut unique_ref = &x;
    println!("&mut x = {:p}", unique_ref );
    println!("&&mut x = {:p}", &unique_ref );

}