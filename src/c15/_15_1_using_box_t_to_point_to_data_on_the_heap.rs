//cargo run 15.1
pub fn main() {
    /*
    Box
    最简单的智能指针，值放在堆上而不是栈上。栈上只是指向堆数据的指针。
    （这有点像c#中，将值类型，装箱成引用类型）
    Box 实现了 Deref trait，允许Box的值被当成引用对待。
    Box 实现了 Drop trait，box值离开作用域时，堆数据会被清除
    */

    let b = Box::new(5);
    println!("b={b}");

    /*
    递归类型：值的一部分可以是相同类型的另一个值。比如链表
    rust需要在编译期知道类型的大小，所以对于递归类型无法计算其大小。必须通过box来创建递归类型.
    enum 类型的大小计算取决于最大成员的空间，内部应该类似于c中的union
    cons list.
    */

    // recursive type `List` has infinite size
    // 报错说List是递归类型，拥有无限大的空间，应该使用Box。Rc等智能指针
    // enum List{
    //     Cons(i32,List), // Cons枚举，包含当前值，和下一个List
    //     Nil,
    // }


    use List::{Cons, Nil};

    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));

}

enum List{
    Cons(i32,Box<List>),
    Nil,
}