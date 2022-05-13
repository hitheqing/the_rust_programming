mod c2;
mod c3;

use std::process::{Command};


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("please use like: 'cargo run 3_1'");
        c2::main();

        Command::new("cmd").arg("/c").arg("pause").status();
        return;
    }

    match (&args[1]).as_str() {
        "2" => c2::main(),
        "3_1" => c3::main_3_1(),
        "3_2" => c3::main_3_2(),
        _ => {}
    }
}
