// https://doc.rust-lang.org/std/rc/struct.Weak.html

use std::{
    ptr,
    rc::{Rc, Weak},
};

pub fn example_1() {
    let strong = Rc::new("hello".to_string());
    let weak = Rc::downgrade(&strong);

    println!("{:?}", ptr::eq(&*strong, weak.as_ptr()));
    assert!(ptr::eq(&*strong, weak.as_ptr()));

    println!("{:?}", unsafe { &*weak.as_ptr() });
    assert_eq!("hello", unsafe { &*weak.as_ptr() });
}

pub fn example_2() {
    let strong = Rc::new("hello".to_owned());
    let weak = Rc::downgrade(&strong);
    let raw = weak.into_raw();

    add_referent_counter(Rc::clone(&strong));

    println!("{:?}", unsafe { &*raw });
}

fn add_referent_counter(s: Rc<String>) {
    println!("count: {:?}", Rc::weak_count(&s));
}
