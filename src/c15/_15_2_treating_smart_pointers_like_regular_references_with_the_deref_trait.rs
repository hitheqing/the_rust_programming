use std::ops::Deref;

pub fn main() {
    /*
    Deref trait，将智能指针当成引用处理
    */

    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let yy = Box::new(5);
    assert_eq!(*yy, 5);

    /*
    自己实现MyBox  达到同样的效果
    */

    let zz = MyBox::new(5);
    assert_eq!( * zz, 5);
}

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
