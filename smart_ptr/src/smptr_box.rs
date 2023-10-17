

fn gen_static_str() -> &'static str {
   let mut s = String::new();
   s.push_str("hello world");

   Box::leak(s.into_boxed_str())
   
}

fn gen_box_str() -> Box<str> {
   let mut s = String::new();
   s.push_str("hello world");

   s.into_boxed_str()
   
}

#[test]
fn test() {
   let s = gen_static_str();
   println!("{}", s);

   let d = gen_box_str();
   println!("{}", d);
   println!("{}", d);
}