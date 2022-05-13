fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

pub fn main() {
    // 这里必须要有类型标注，否则不知道应该parse成什么。也可以有上下文来推断
    let guess: u32 = "42".parse().expect("Not a number!");
    assert_eq!(guess, 42);

    //scalar-integer
    //i8 i16 i32 i64 i128 isize   默认类型为i32
    //u8 u16 u32 u64 u128 usize
    println!("\n标量-整数");
    let i = 0;
    println!("let i = 0; 默认类型:{:?}", type_of(&i));
    println!("支持下划线分割 十进制 {}:{}", "98_222", 98_222);
    println!("支持下划线分割 二进制 {}:{}", "0b1111_0000", 0b1111_0000);

    //scalar-floating
    //f32 f64 默认f64
    // let f = .0; // 不可以c++中省略0
    println!("\n标量-浮点数");
    let f = 0.0;
    let f2: f32 = 0.0;
    println!("let f = 0.0; 默认类型:{:?}", type_of(&f));
    println!("let f2:f32 = 0.0; type of f2 is:{:?}", type_of(&f2));

    //数字运算，类型必须一致
    println!("\n数字运算");
    println!("数字运算类型必须一致 let div = 3/2.0; 整形 op 浮点型 无法通过编译");

    //bool
    println!("\nbool类型");
    println!("type of true is:{},type of false is:{}", type_of(&true), type_of(&false));

    // char
    println!("\n字符类型");
    let heart_eyed_cat = '😻';
    println!("let heart_eyed_cat = '😻';type:{}", type_of(&heart_eyed_cat));

    // tuple
    println!("\n元组类型");
    let position = (10, 20);
    let (x, y) = position; // 元组解构
    println!("let position = (10,20); 元组声明后无法改变大小和类型。 用下标来访问元素 position.0 = {} position.1 = {} ", position.0, position.1);
    println!("let (x,y) = position;  元组可以被解构成多个值。 x = {} ,y = {} ", x, y);
    println!("()代表单元值。如果表达式不返回其他值，隐式返回单元值");

    // array
    println!("\n数组类型");
    let array_3 = [1, 2, 3];
    let array_2 = [2; 3];
    println!("let array_3 = [1,2,3];  列表初始化  array_3 is:{:?}", array_3);
    println!("let array_2 = [2;3];  n个相同元素初始化  array_2 is:{:?}", array_2);
    println!("通过下标访问。不可越界  array_3[2] is:{:?}", array_3[2]);
    ()
}