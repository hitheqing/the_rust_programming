pub fn main() {
    println!("match 一个枚举变量时，其分支必须是穷尽的， 配合ide一起使用 (add remain patterns)，可以生成匹配穷举");
    println!("match 大括号也是一个表达式，会返回一个值。 match的所有分支都应该返回相同类型的值");

    println!(
        "match 绑定值模式。 如果枚举有关联变量，在生成分支时，会同时被绑定。在6.1中已经演示过 "
    );
    println!(
        "match 穷尽，使用 通配符 _ => () 来表示其他情况，类似于 c++ C#中switch 的  default语句。 "
    );

    // match的穷尽完美的解决我之前的痛点.  SLG游戏中有10多种地块类型,在很多地方都要进行switch判断. 每次新曾一种类型,需要在很多地方检查switch 新增处理分支.
    // 而人力检查容易漏掉,rust直接在编译层面 不让过, 非常好用
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    let a: u32 = match coin {
        // 尝试把下面的分支注释掉，ide会报错。然后点击 add remain patterns 来生成分支。
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter => 4,
    };
    a
}
