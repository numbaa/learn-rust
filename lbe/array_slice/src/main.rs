use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elemenmts", slice.len());
}

fn main() {
    //都是数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    let zs = [233; 4];
    println!("{:?}", zs);
    println!("array occupies {} bytes", mem::size_of_val(&zs));
    println!("borrow bthe whole array as a slice");
    //array似乎一borrow就是slice?
    analyze_slice(&xs);
    //如果&ys[1..3]是slice，那ys[1..3]是啥？
    analyze_slice(&ys[1..3]);
    //编译就错误了
    //println!("{}", xs[8]);
    //panic
    println!("{}", xs[2..4][3]);
}