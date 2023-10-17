///
use std::ops::Drop;
struct HasDrop1;
struct HasDrop2;

impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HashDrop1!");
    }
    
}

impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Dropping HashDrop2!");
    }
    
}

struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Dropping HasTwoDrops!");
    }
}

struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!")
    }
}

//error: impl Copy for Foo { 
    
// }

#[test]
fn test() {
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    let _foo = Foo;
    println!("Running!");

}

#[test]
fn test_1() {
    let mut _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };

    //error: _x.drop();
    drop(_x);
}