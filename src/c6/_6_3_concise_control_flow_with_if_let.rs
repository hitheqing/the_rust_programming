pub fn main(){
    println!("if let  空值流 用于处理只需要关心某一种模式的情况. 是match的一种语法糖");
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    };

    if let Some(3) = some_u8_value {
        println!("three");
    };

    println!("let some_u8_value = Some(0u8);
    match some_u8_value {{
        Some(3) => println!(\"three\"),
        _ => (),
    }}");
}