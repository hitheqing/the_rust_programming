pub fn main()
{
    println!("\n函数通常指的是c语言那样没有具体对象的函数。而方法通常指的是结构体上的方法。");
    {
        println!("结构体字段定义和 方法定义分开来。使用impl关键字为结构体实现方法,并且可以分多个块来写");
        println!("方法语法中第一个参数必须为&self。如果需要move，则申明成self。如果需要改变，则&mut self");

        let mut rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        // let r = rect1.area_move();
        // let r2 = rect1.area(); // r已经被move了，后续无法再调用方法
        rect1.change_width();

        println!("let mut rect1 = Rectangle {{
    width: 30,
    height: 50,
}};

// let r = rect1.area_move();
// let r2 = rect1.area(); // r已经被move了，后续无法再调用方法
rect1.change_width();
println!(\"{{}}\",rect1.width);");

        println!("{}", rect1.width);
    }

    println!("\n关联函数, 定义在impl块中，却不以self为第一个参数，类似于c#中的静态方法。 使用::调用。");
    println!("impl Rectangle{{
    fn square(size: u32) -> Rectangle {{
        Rectangle {{
            width: size,
            height: size,
        }}
    }}
}}
let rect = Rectangle::square(20);
");

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn area_move(self) -> u32 {
        self.width * self.height
    }

    fn change_width(&mut self) {
        self.width = self.width * 2;
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}