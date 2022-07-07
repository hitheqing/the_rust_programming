pub fn main() {
    // 生命周期也是泛型
    // 生命周期与引用有效性

    /*
    生命周期注解并不改变任何引用的生命周期的长短。与当函数签名中指定了泛型类型参数后就可以接受任何类型一样，
    当指定了泛型生命周期后函数也能接受任何生命周期的引用。生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。
    */
    //

    test();
}

//'a 就是 生命周期的泛型，仅仅描述多个生命周期的关系，而不影响生命周期
// 单个生命周期没有意义，多个引用的生命周期大小才有意义
// 仅仅编译期检查，不满足生命周期的参数传入不会通过编译
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 下面不能编译。  函数返回生命周期，但是函数内部创建的值，会在函数结束时离开作用域，变成了垂悬引用
// 返回生命周期参数需要与其中一个参数的生命周期匹配
// 像如下的情况，应该返回 所有权类型，而不是返回引用类型。 返回的值的所有权由调用者负责清理（作用域结束自动清理）
//fn longest22<'a>(x: &str, y: &str) -> &'a str {
//    let result = String::from("really long string");
//    result.as_str()
//}

//生命周期结构体
#[derive(Debug)]
struct Example<'a, 'b> {
    //存储引用，需要lifetime。表示该结构的实例  存活时间必须小于等于 其字段的存活时间。
    //存活时间表示  先有字段，然后才有结构体对象；结构体对象先析构，然后字段析构
    aaa: &'a str,
    bbb: &'b str,
}

fn test() {
    //    let st :&'static str = "sss";
    // 以下不能通过编译，因为 对aaa赋值的引用 来自于s，其生命周期小于 结构的生命周期
    //    let mut e:Example = Example{ aaa: "", bbb: "" };
    //    {
    //        let s = "aaa".to_string();
    //        let aaa = s.as_str();
    //        e.aaa = aaa;
    //    }
    //    println!("e is:{:?}", e);
}

//lifetime 省略规则
//1. 每个引用参数都有生命周期 参数
//2. 如果只有1个引用输入，那么引用输出来自于引用输入
//3. 如果有多个引用输入，但是有&self or &mut self,那么所有输出都使用self的引用输入参数
// 第三个规则应用于  结构体方法中。因为通常都是&self or &mut self

//static lifetime  相当于全局变量
//let st :&'static str = "sss";
