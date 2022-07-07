pub fn main() {

    /*
    1.默认多线程并行测试。所以测试之间不能互相依赖、共享状态。比如同时读写文件。
    2.--test-threads=1  指定测试的线程数量
    3.如果调用了print，成功的测试不会打印内容。只有失败的才会。使用 --nocapture来让成功的也打印
    */

    /*
    1.使用#[ignore]忽略测试，会在ide中显示
    */
}
#[cfg(test)]
mod mytest{
    #[test]
    fn ttt_succ(){
        println!("ttt_succ");
        assert_eq!(1,1);
    }

    #[test]
    #[ignore] // ignore 会出现在ide下方。
    fn ttt_failed(){
        println!("ttt_failed");
        assert_eq!(1,2);
    }
}

/*
:cargo test
---- c11::_11_2_controlling_how_tests_are_run::mytest::ttt_failed stdout ----
ttt_failed
thread 'c11::_11_2_controlling_how_tests_are_run::mytest::ttt_failed' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src\c11\_11_2_controlling_how_tests_are_run.rs:20:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


:cargo test -- --nocapture
打印所有print内容，不管是否失败

:cargo test -- --ignored
只运行带有ignore的测试

*/