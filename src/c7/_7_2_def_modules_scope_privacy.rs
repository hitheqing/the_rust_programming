pub fn main(){
    println!("以mod 来划分模块，类似于电脑的文件目录树");
    println!("
crate
└── sound
    ├── instrument
    │   └── woodwind
    └── voice");
    println!("借助ide，鼠标放在mod上可以显示mod的绝对路径。 绝对路径从跟开始");
    println!("绝对路径  crate::c7::_7_2_def_modules_scope_privacy::sound::voice");
    println!("相对路径  self::sound::voice");
    println!("使用pub关键字 来公开模块。pub关键字 适用于 结构体，枚举，函数，方法，模块");
    println!("使用super关键字 来访问父级 mod");
    println!("对于结构体，需要为每个字段指定 pub 公开性。而对于枚举，默认就是公开的，一个私有的枚举没有意义。");
    println!("使用use 关键字，类似于文件系统中创建软连接。");
    println!("  对于函数来说，use指定到函数的所属模块");
    println!("  对于结构体、枚举和其他，通过use指定全路径。习惯性做法。");
    println!("  如果2个模块中有同名结构，那么就需要区分开");
    println!("使用as关键字给引入的类型重命名。这个在c#，python等中也有，c++中就是 宏定义");

    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert(1, 2);
}

mod sound {
    mod instrument {
        mod woodwind {
            fn clarinet() {
                // 函数体
            }
        }
    }

    pub mod voice {

    }
}
