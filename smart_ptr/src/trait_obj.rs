///

trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id)
    }
}

struct Select {
    id: u32,
}

impl Draw for Select {
    fn draw(&self) {
        println!("这个选择框贼难用{}", self.id)
    }
}

#[test]
fn test_trait_obj() {
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button{id: 1}), Box::new(Select{id: 10})];

    for e in elems {
        e.draw();
    }
}
