// 需要use HashMap
use std::collections::HashMap;

pub fn main() {
    // 构造实例
    let mut scores = HashMap::new();

    //add
    scores.insert(1, 10);
    scores.insert(2, 20);
    scores.insert(3, 30);

    print_map(&scores);

    //remove
    scores.remove(&1);

    print_map(&scores);

    //modify
    let mut finded = scores.get_mut(&3).unwrap();
    *finded = 33;

    print_map(&scores);

    // contains
    let b = scores.contains_key(&2);
    println!("b is:{:?}", b);

    // insert if not exist 仅仅没有时插入,注意这里会发生move
    scores.entry(1).or_insert(10);
    scores.entry(4).or_insert(40);
    print_map(&scores);

    //分别初始化
    let keys = vec![1, 2, 3];
    let values = vec![10, 20, 30];
    let mut kvs: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    println!("将值的引用插入 集合，值本身不会移动，但是值必须再容器有效时有效、意思是通过容器访问值和直接访问值应该要有相同的预期");
}

fn print_map(map: &HashMap<i32, i32>) {
    for (k, v) in map {
        println!("k={},v={}", k, v)
    }
}

// fn print_map (map :HashMap){
//
// }
