// https://doc.rust-lang.org/std/rc/struct.Weak.html

use std::{
    ptr,
    rc::{Rc, Weak},
};

pub fn demo_downgrade_get_weak() {
    let strong = Rc::new("hello".to_string());
    let weak = Rc::downgrade(&strong);

    println!("{:?}", ptr::eq(&*strong, weak.as_ptr()));
    assert!(ptr::eq(&*strong, weak.as_ptr()));

    println!("{:?}", unsafe { &*weak.as_ptr() });
    assert_eq!("hello", unsafe { &*weak.as_ptr() });
}

pub fn demo_into_raw() {
    let strong = Rc::new("hello".to_owned());
    let weak = Rc::downgrade(&strong);

    let raw = weak.into_raw();
    drop(unsafe {Weak::from_raw(raw)});
    println!("count weak: {:?}", Rc::weak_count(&strong));
    println!("count strong: {:?}", Rc::strong_count(&strong));

    println!("{:?}", unsafe { &*raw });
}

