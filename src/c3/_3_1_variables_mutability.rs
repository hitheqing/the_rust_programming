pub fn main() {
    // x 是不可变变量  不可以改变
    let x = 2;
    // x = 6;  // x 是不可变变量  不可以改变
    print!("let x = 2; x 是不可变变量  不可以改变. x = {}", x);

    // y 可以被改变,但是类型不可变
    let mut y = 4;
    print!("let mut y = 4; y 可以被改变,但是类型不可变. y = {}", y);
    y = 5;
    // y = "y";
    print!("y = 5;  y = {}", y);

    // 常量，大写下划线，类型注释，编译期可计算值
    const MY_AGE: i32 = 30;
    print!("const MY_AGE: i32 = 30; 常量，大写下划线，类型注释，编译期可计算值. MY_AGE = {}", MY_AGE);


    // shadowing 变量遮蔽，重新绑定值和类型
    let m = 2;
    print!("let m = 2; m = {}", m);
    let m = "324";
    print!("let m = \"324\"; 重新绑定值和类型 m = {}", m);
}