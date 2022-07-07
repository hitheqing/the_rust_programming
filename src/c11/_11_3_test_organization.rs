pub fn main() {

    /*
    单元测试
    1.单元测试模块使用#[cfg(test)]标记，且放到同一个文件中
    2.#[cfg(test)]，只有cargo test时才编译，而cargo build不会编译这部分代码

    集成测试
    1.测试的库相当于外部库。使用库文件，调用共有api
    2.在src同目录创建tests目录
    3.在tests下的每个文件都会当作单独的crate编译,并且不需要添加#[test]宏
    4.test目录也是只有在cargo test时才编译
    */

    /*
    main.rs 应该只有少量的代码
    lib.rs中应该包含大部分库
    测试只能测试lib.rs中的mod和函数
    */
}
