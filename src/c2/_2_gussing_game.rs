use std::cmp::Ordering;
use rand::Rng;

pub fn main() {
    let rand_number = rand::thread_rng().gen_range(1..101);
    let mut guess_count = 0;
    loop {
        println!("请输入要猜的数字");
        let mut guess = String::new();

        // 读取输入。 链式调用换行 可以借助ide查看每个调用的返回
        std::io::stdin()
            .read_line(&mut guess)
            .expect("fail to read");

        let number: i32 = guess.trim()
            .parse()
            .expect("failed to parse to number");

        println!("你输入的是:{}", guess);

        guess_count += 1;

        match number.cmp(&rand_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("猜对了！总共猜了{}次。", guess_count);
                break;
            }
        };
    }
}