pub fn main() {
    //所有权规则
    println!("\n所有权规则");
    println!("1.每个值任意时刻只有一个owner");
    println!("2.owner离开作用域后，值被丢弃。后续无法再访问");
    let _a = 2;
    println!("let _a = 2;  把值2绑定到变量_a, _a就是这个值的owner");

    println!("\n变量作用域scope");
    println!("跟其他语言类似，花括号范围内生效");

    println!("\nString类型");
    println!("1.字符串面量不可变");
    println!("2.String类型可变，分配在堆上。");
    println!("  2.1.堆上的对象需要运行时由分配器请求内存。String::from");
    println!("  2.2.变量不再使用时将内存返回给分配器。owner离开作用域后释放。自动调用drop");

    println!("\nmove");
    {
        println!("对于简单类型和实现了Copy Trait的类型，赋值和函数调用会执行拷贝");
        let x = 5; // 将5绑定到x
        let y = x; // 将值x拷贝一份，并且绑定到y。 相当于stack上有2个5
        println!("let x = 5;// 将5绑定到x");
        println!("let y = x;// 将值x拷贝一份，并且绑定到y。 相当于stack上有2个5");
    }
    {
        println!("对于其他类型,赋值和函数调用会执行move");
        let s1 = String::from("hello"); // 将字符串绑定到s1
        let s2 = s1; // 将值x拷贝一份，并且绑定到y。 相当于stack上有2个5
        println!("let s1 = String::from(\"hello\");// 将hello绑定到s1");
        println!("let s2 = s1;// 将s1 move 到s2，s1不能再被使用");
    }

    println!("\nclone");
    {
        println!("如果确实需要深拷贝，调用 clone函数");
        let s1 = String::from("hello");
        let s2 = s1.clone();
        assert_eq!(s1, s2);
    }

    println!("\nCopy Trait");
    {
        println!("如果类型实现了Drop trait，那么不能实现copy trait。由该类型组合的其他类型也无法实现copy trait");
        println!("所有标量类型实现了copy");
        println!("元组类型的每个子类型实现copy，自身才会被copy。eg. (i32,i32)有，(i32,String)没有");
        let s1 = String::from("hello");
        let s2 = s1.clone();
        assert_eq!(s1, s2);
    }

    println!("\n所有权和函数");
    {
        println!("函数传参和普通赋值一样，也会发生移动或者复制");
        println!("规则和赋值一样，类型决定了发生move 还是 copy");
        println!("fn foo(will_move: String,will_copy: i32) 这个函数调用时，will_move 参数会move，will_copy参数会copy");
        println!("函数返回值可以转移owner，在函数内创建变量，并且返回，就是move所有权");
    }
}
