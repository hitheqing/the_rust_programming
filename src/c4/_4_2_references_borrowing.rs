pub fn main() {
    println!("\nreference borrowing 引用与借用");
    {
        println!(
            "大多情况下函数调用会转移所有权导致变量在函数调用后无法访问，\
        需要把参数再次返回并且重新绑定变量才可以。使用引用来解决这一问题"
        );
        println!("变量前加&就是定义一个引用。函数参数改为引用类型，就可以使用值，而不获取所有权");
        println!("fn foo(a_ref:&String,b_ref: &i32) a_ref,b_ref将以引用传入。");
        println!("创建引用的行为称为borrowing借用");
    }

    println!("\n可变引用");
    {
        let mut a = 2;
        let b = &mut a;
        *b = 100;
        println!(
            "let mut a = 2;
let  b = &mut a;
*b = 100;
assert_eq!(a,100);"
        );
        assert_eq!(a, 100);

        println!("同一时间只能有1个可变引用。因为多个可变引用可能出现数据竞争.同一时间指的是同一个作用域");
        println!("不能同时拥有可变引用和不可变引用。因为不可变引用不希望发生意外的改变。（如果不可变在前，可变在后是可以的，不过还是注意使用最好不要出现）");
        println!("可以同时拥有多个不可变引用。因为只读不影响其他只读。");
    }

    println!("\n悬垂引用 dangling references");
    {
        println!("c++中，如果释放一块内存，但是保留了指向它的指针，就会变成悬空指针。rust中不会出现悬垂引用，对于返回引用的情况需要指定生命周期");
    }
}
