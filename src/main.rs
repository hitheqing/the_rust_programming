mod c2;
mod c3;
mod c4;
mod c5;
mod c6;
mod c7;
mod c8;
mod c9;
mod c10;
mod c11;
mod c12;
mod c13;
mod c14;
mod c15;
mod c16;
mod c17;
mod c18;
mod c19;
mod c20;

use std::process::{Command};


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("please use like: 'cargo run 3_1'");
        c2::main();

        Command::new("cmd").arg("/c").arg("pause").status().unwrap();
        return;
    }

    match (&args[1]).as_str() {
        "2" => c2::main(),
        "3_1" => c3::main_3_1(),
        "3_2" => c3::main_3_2(),
        "3_3" => c3::main_3_3(),
        "3_4" => c3::main_3_4(),
        "3_5" => c3::main_3_5(),
        "4_1" => c4::main_4_1(),
        "4_2" => c4::main_4_2(),
        "4_3" => c4::main_4_3(),
        "5_1" => c5::main_5_1(),
        "5_2" => c5::main_5_2(),
        "5_3" => c5::main_5_3(),
        _ => {}
    };
}
