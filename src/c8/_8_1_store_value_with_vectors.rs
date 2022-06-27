pub fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(0);
    let first = vec.first().unwrap();
    vec.push(1);
    // println!("first:{}",first);

    //first是vec的不可变借用，vec.push发生了可变借用。无法通过编译
    println!("first是vec的不可变借用，vec.push发生了可变借用。无法通过编译");
    println!("集合的使用原则：现取现用，不能延迟到集合发生改变，可能引起内存扩容变化");

    // 添加
    vec.push(2);
    // 移除
    vec.pop();
    vec.remove(0);

    {
        let vec2 = vec![1, 2, 3]; // vec列表初始化
        println!("vec2脱离作用域后，内部元素会被丢弃。");

        // 访问
        let a = &vec2[0];
        let aa = vec2.get(0);
        println!("使用&[0]来获取vec中元素的不可变引用，或者使用get(0)来获取一个Option。");
        println!("当数组越界时，[0]会panic，而get返回None");
    }

    // 遍历,应该使用 &vec 或者 &mut ,否则会发生move
    for v in &vec {}

    // vec只能存储相同类型。但是可以存储枚举，通过枚举来存储多种类型
}
