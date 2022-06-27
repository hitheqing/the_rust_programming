pub fn main() {
    //     预期内的可控的错误，应该使用result
    //    不可修改的结果使用panic

    // 通过构造一个对象来校验输入的合法性，可能会比较多余，但是在某些重要场合是必要的。把错误屏蔽在构造阶段。
    // 然后通过getter方法来真正返回需要的值
}


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}