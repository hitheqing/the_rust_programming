pub fn main(){
    println!("cargo new 创建一个包 crate");
    println!("Cargo.toml 描述如何构建一个或多个crate");
    println!("./src/main.rs   ./src/lib.rs 为 crate的根root。 ");
    println!("一个包 0个或者1个库，取决于有没有 lib.rs. 如果同时包含main和lib，则有2个crate。");
    println!("./src/bin 下每个文件都是一个单独的二进制crate。");


    /*
    ---src
        |---main.rs      二进制文件跟
        |---lib.rs       库文件跟
        |---bin
            |---xxx       每个文件都是二进制文件跟
    ---Cargo.toml
    */
}