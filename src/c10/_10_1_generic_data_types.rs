pub fn main() {
    // 泛型
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    let result2 = largest222(&number_list);
    println!("result is:{:?}", result);
    println!("result2 is:{:?}", result2);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest222(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
