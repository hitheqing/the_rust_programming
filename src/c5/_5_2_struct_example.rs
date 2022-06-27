pub fn main() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    };

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{{:?}} 输出在一行，适用于简单结构打印 rect1 is {:?}", rect1);
    println!("{{:#?}}输出多行，适用于复杂结构打印 rect1 is {:#?}", rect1);
    dbg!(&rect1);
    println!("dbg!(&rect1); 宏函数，输出多行的同时打印文件名和行号");
}
