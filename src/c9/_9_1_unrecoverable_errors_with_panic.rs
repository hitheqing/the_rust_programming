pub fn main() {
    println!("如果使用 CMD: set RUST_BACKTRACE=1");
    println!("如果使用 powershell: $Env:RUST_BACKTRACE=1");
    println!("IDE中的Terminal，看起来是PS（powershell），所以运行本例子的命令为   $Env:RUST_BACKTRACE=1; cargo run 9_1 ");

    // 不知道如何处理的错误
    ppp();
}

fn ppp() {
    panic!("crash and burn");
}
