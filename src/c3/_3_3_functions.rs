pub fn main() {
    // 函数调用
    another_function();
    another_function_args(5);
    let ret = another_function_args_returns(5);
    println!("return of another_function_args_returns is:{:?}", ret);

    // 语句和表达式
    println!("语句不返回值。let x = 1; 是一个语句");
    println!("表达式返回值，不以 ; 结尾。  () 是表达式，返回单元值   {{}} 也是表达式，返回里面的内容。 100 也是表达式，返回100");
    let a = if 0 == 0 { 2 }  else { 3 } ;
    println!("三元运算符再rust中的写法。 let a = if 0 == 0 {{ 2 }}  else {{ 3 }} ; a:{}",a);
    ()
}

fn another_function() {
    println!("another_function.");
}

fn another_function_args(x: i32) {
    println!("another_function_args. x is {}", x);
}

fn another_function_args_returns(x: i32) -> String {
    println!("another_function_args_returns. x is {}", x);
    return "hello return".to_string();
}