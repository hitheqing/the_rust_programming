pub fn main() {
    // 如何编写测试

    /* 测试步骤
    1.构造输入参数
    2.运行测试代码
    3.断言输出结果
    */

    /*
    1.函数添加 #[test]
    2.cargo test, 会运行 上述函数
    3.借助ide，运行单个测试
    */
}

// 可以直接在ide上运行某个测试
#[cfg(test)]
mod mytest {
    #[test]
    pub fn success_test() {
        assert_eq!(1 + 2, 3); // 这个可以通过
    }

    //    #[test]
    //    pub fn fail_test(){
    //        assert_eq!(1+2,4); // failed
    //    }

    //use Result<T,E> for test
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
