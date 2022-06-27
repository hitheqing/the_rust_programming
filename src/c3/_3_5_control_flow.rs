pub fn main() {
    // if
    println!("if");
    {
        let number = 3;
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    };
    println!("if 的条件必须是bool值。不能像c++中的if 1");
    // let a = if true { 3} else { 3.0 };
    println!(
        "if 是表达式，每个分支必须返回相同的值。 if true {{ 3}} else {{ 3.0 }};  无法通过编译"
    );

    // loop
    println!("\nloop\nloop类似于while true");
    println!("loop vs while true。 loop会保证至少执行1次，而后者不会。  https://stackoverflow.com/questions/28892351/what-is-the-difference-between-loop-and-while-true");
    /*
    {
        let x;
        loop { x = 1; break; }
        println!("{}", x)
    };
    // 下面这段无法通过编译
    {
        let x;
        while true { x = 1; break; }
        println!("{}", x);
    };
    */

    //loop label
    println!("可以在loop前面加上标签。然后break continue一起使用，可以应用于该层循环，而不是只能应用于内层循环。很适合用来跳出多重循环!!");
    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;

            loop {
                println!("remaining = {}", remaining);
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {}", count);
    };

    //loop return
    println!("loop 可以在break中添加返回值. 和 if一样，多个break必须返回类型一致");
    {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
            // break ""; // 比如这里无法通过编译
        };

        println!("The result is {}", result);
    };

    // while
    println!("\nwhile");
    {
        let mut number = 3;

        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }

        println!("LIFTOFF!!!");
    }

    // for
    println!("\nfor");
    {
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    };
}
