pub fn main() {
    println!("\n结构体定义");
    {
        println!("struct关键字，类型标注，逗号分隔");
        println!(
            "struct People{{
    name:String,
    age:i32,
}}"
        )
    }

    println!("\n结构体实例化");
    {
        let p = People {
            name: "".to_string(),
            age: 0,
        };
        println!("let p = People{{ name: \"\".to_string(), age: 0 }};");
        println!("结构体跟普通变量一样，也分为可变和不可变。不可变的无法修改字段值");
        println!("  1.构造实例多个参数顺序可以不一致，推荐使用ide来完成构造");
        println!("  2.如果构造实例的参数名和字段名一致，则可以省略。");
        println!("\n从一个值创建另一个实例。 let p2 = People{{age:21,..p}}; ..放在最后面，其他字段放前面");
        let p2 = People { age: 21, ..p };
        println!("结构体更新语法  let p2 = People{{age:21, ..p}}; p2.age = {}。 和赋值，函数调用一样，也会触发移动或者拷贝。比如这里p已经不可以访问了。", p2.age);
        // println!("{:?}",p);
    }

    println!("\n没有字段名字的元组结构体");
    {
        let point = Point2(3, 4);
        println!("定义：struct Point2(i32,i32);");
        println!("实例化：let point = Point2(3,4);");
        println!("访问：和元组一样使用下标。point.0 {}", point.0);
        let Point2(x, y) = point;
        println!(
            "解构：和元组类似，但是要写类型。let Point2(x,y) = point; x = {},y = {}",
            x, y
        );
    }

    println!("\n没有任何字段的类单元结构体");
    {
        println!("定义：struct UnitStruct;");
        println!("实例化：let us = UnitStruct;");
    }

    println!("\n字段所有权");
    {
        println!("结构体中的字段如果是 没有所有权类型(引用，切片), 需要加上生命周期，确保引用的数据有效性 和 结构体本身保持一致。在第10章深入生命周期");
    }
}

#[derive(Debug)]
struct People {
    name: String,
    age: i32,
}

struct Point2(i32, i32);

struct UnitStruct;
