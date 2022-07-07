use std::rc::Rc;

pub fn main() {
    /*
    使用Rc  引用计数 智能指针
    */

    // box无法解决 多个引用同时存在的问题，会move。所以使用Rc来完成这一任务
    //    let a = MySt(5);
    //    let b = Box::new(a);
    //    let c = Box::new(a);

    let b = Rc::new(5);
    println!("strong_count is:{:?}", Rc::strong_count(&b)); //1
    let c = Rc::clone(&b);
    println!("strong_count is:{:?}", Rc::strong_count(&b));//2
    let d = Rc::clone(&c);
    println!("strong_count is:{:?}", Rc::strong_count(&b));//3
    println!("strong_count is:{:?}", Rc::strong_count(&c));//3
    println!("strong_count is:{:?}", Rc::strong_count(&d));//3
    // Rc::strong_count  只计算最原始的Rc对象被引用了多少

    let aa = Rc::new(5);
    assert_eq!(*aa,5); //Rc 实现了Deref，依然可以想Box一样，自动解引用

    /*
impl<T: ?Sized> Deref for Rc<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        &self.inner().value
    }
}
    */

}

struct MySt(i32);
