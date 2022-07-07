use std::collections::HashMap;
use std::fs::copy;
use std::hash::Hash;
use std::iter::Map;

pub fn main() {
    /*
    闭包：
        1.匿名函数，作为参数传递，作为变量存储
        2.捕获作用域中的值

        3.编译器推断闭包的类型。
            输入参数类型、返回结果类型
            闭包本身类型，捕捉的变量，可变还是不可变等

    */

    //如果只有这一句，则会报错，因为编译器无法推断输入参数和输出结果的类型
    let example_closure = |x| x;

    //有了闭包调用，在这里可以推断出类型
    let s = example_closure(String::from("hello"));

    //但是下面这个 不可以调用，因为与已经确定了的闭包类型参数不符合、
    //    let n = example_closure(5);

    /*
    lazy evaluation  惰性求值，将计算结果存储起来，第二次调用时直接取值
    */
}

//泛型结构，T为匿名类型。类似C#中的Action，Func
//Fn，FnMut。FnOnce
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // 计算体
    calculation: T,
    // 计算结果
    value: Option<u32>,
}

// 为泛型结构定义方法，impl需要加<T>
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    //惰性求值。
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

//上面的结构有限制，不能根据不同的输入返回不同的结果。下面是改进版，使用map存储不同的value
struct CacherV2<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value_map: HashMap<u32, u32>,
}

impl<T> CacherV2<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> CacherV2<T> {
        CacherV2 {
            calculation,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value_map.get(&arg) {
            None => {
                let v = (self.calculation)(arg);
                self.value_map.insert(arg, v);
                v
            }
            Some(v) => *v,
        }
    }
}

//上面的v2依然有缺陷，只能输入u32返回u32，可以针对不同的输入 返回不同的输出。下面是改进版v3
struct CacherV3<T, U, V>
where
    T: Fn(U) -> V,
{
    calculation: T,
    value_map: HashMap<U, V>,
}

impl<T, U, V> CacherV3<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    fn new(calculation: T) -> CacherV3<T, U, V> {
        CacherV3 {
            calculation,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value_map.get(&arg) {
            None => {
                let v = (self.calculation)(arg);
                self.value_map.insert(arg, v);
                v
            }
            Some(v) => *v,
        }
    }
}
