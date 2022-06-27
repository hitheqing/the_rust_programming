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
mod c2;
mod c20;
mod c3;
mod c4;
mod c5;
mod c6;
mod c7;
mod c8;
mod c9;

use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("please use like: 'cargo run 3_1'");
        c2::main();

        Command::new("cmd").arg("/c").arg("pause").status().unwrap();
        return;
    }

    let chapter =  args[1].as_str();
    let mut c :String = chapter.to_string();
    if chapter.contains(".") {
         c = chapter.replace(".","_")
    }

    match c.as_str() {
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
        "6_1" => c6::main_6_1(),
        "6_2" => c6::main_6_2(),
        "6_3" => c6::main_6_3(),
        "7_1" => c7::main_7_1(),
        "7_2" => c7::main_7_2(),
        "7_3" => c7::main_7_3(),
        // "7_4" => c7::main_7_4(),
        // "7_5" => c7::main_7_5(),
        "8_1" => c8::main_8_1(),
        "8_2" => c8::main_8_2(),
        "8_3" => c8::main_8_3(),
        "9_1" => c9::main_9_1(),
        "9_2" => c9::main_9_2(),
        "9_3" => c9::main_9_3(),
        "10_1" => c10::main_10_1(),
        "10_2" => c10::main_10_2(),
        "10_3" => c10::main_10_3(),
        _ => {}
    };
}
