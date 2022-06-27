pub fn main() {
    println!("\n切片 slice");
    {
        println!("和引用一样，没有所有权，引用集合中一段连续的元素序列，而不引用整个集合");
        println!("和python语法类似，使用..来表示下标范围。可以省略0和最大长度，使用=表示闭区间");
        let s = String::from("helloworld");
        println!("let s = String::from(\"helloworld\");");
        println!("&s[0..5] is {:?}", &s[0..5]);
        println!("&s[..5] is {:?}", &s[..5]);
        println!("&s[6..10] is {:?}", &s[6..10]);
        println!("&s[6..] is {:?}", &s[6..]);
        println!("&s[0..=5] is {:?}", &s[0..=5]);
    }

    println!(
        "字符串字面量就是slice &str,指向该位置的slice。type_of(\"xxx\"):{}",
        type_of(&"xxx")
    );
    println!("把参数定义成&str，在使用上更加通用");
    println!("数字的slice和字符串类似。详见第八章");
}

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
