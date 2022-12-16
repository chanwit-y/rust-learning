use std::rc::Rc;

pub fn demo_rc_1() {
    let foo = Rc::new(vec![1, 2, 3]);
    let a = foo.clone();
    let b = Rc::clone(&foo);

    println!("{:?}", Rc::strong_count(&foo));
}
