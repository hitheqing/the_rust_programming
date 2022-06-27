use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read, Write};

pub fn main() {
    println!("Result枚举表示可恢复错误。比如文件不存在就创建");

    // 一般错误处理，嵌套match
    {
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
                },
                other_error => panic!("There was a problem opening the file: {:?}", other_error),
            },
        };
    }

    // 使用闭包 lamda 减少match次数
    {
        let f = File::open("hello.txt").map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Tried to create file but there was a problem: {:?}", error);
                })
            } else {
                panic!("There was a problem opening the file: {:?}", error);
            }
        });
    }

    // 使用?错误传播
    let ret = read_username_from_file_222();
}

// 第一版写法
//  Result 是个枚举，返回类型也必须是枚举，所以成功的话应该返回OK(T)，在这个例子中就是OK(String)
fn read_username_from_file_111() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s), // s
        Err(e) => Err(e),
    }
}

// 第二版写法
// 使用？进行传播，返回结果要是Result类型，遇到了错误就马上返回，正确才继续往下执行
// 可以看到，使用？传播的写法，在最后通常会返回OK(T)
fn read_username_from_file_222() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 第三版写法，在上面的基础上，增加链式调用
fn read_username_from_file_333() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
