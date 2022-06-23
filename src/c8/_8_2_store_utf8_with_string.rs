pub fn main() {
    // String 可变字符串。&str，字符串切片
    let mut s = "hello".to_string();
    let s2 = String::from("hello");
    let ss2 = "hello";

    //String更新
    s.push_str(s2.as_str());
    let s3 = s + &s2;
    // println!("s2 is:{:?}", s);
    println!("字符串更新语法，这里方法签名中是self而非 &self，所以调用+后，左边的值被move了。而右边是&str，所以右边的还可以继续使用");
    println!("对于多个连接，使用format!宏，不会获取所有权。");
    // 2个以上的拼接
    let sformat = format!("{}{}{}", 1, 2, 3);


    // 索引字符串不允许,
    let sss = "xyz".to_string();
    // let c = sss[0]; //不能编译，因为utf8编码，一个字符占了多个长度，会带来误会

    // 可转换为chars,bytes后再index
    for v in sss.chars() {
        println!("v is:{:?}", v);
    }

    for v in sss.bytes() {
        println!("v is:{:?}", v);
    }
}