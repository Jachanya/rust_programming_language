use std::rc::Rc;
use std::cell::RefCell;

trait Draw {
    fn draw(&self);
}

fn main() {
    let pointer_x: Box<i32> = Box::new(32);
    let pointer_xy = Rc::new(pointer_x);
    let clone_p = Rc::clone(&pointer_xy);
    println!("clone_p {}", clone_p);
    let pointer_z = RefCell::new(pointer_xy);
    println!("Hello, world! {:#?}", pointer_z);
}
