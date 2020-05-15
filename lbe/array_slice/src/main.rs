#[derive(Debug)]
struct Person<'a> {
    name: &'a str,  //忘了声明周期标注的语法了...
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    let name = "Inm";
    let age = 24;
    let inm = Person{name, age};
    println!("{:?}", inm);

    //大概是最繁琐的写法
    let point: Point = Point {x: 1.0, y: 2.0};
    //这个好
    let new_point = Point{x: 3.0, ..point};
    //牛逼
    let Point{x: new_x, y: new_y} = new_point;
    println!("x:{}, y:{}", new_x, new_y);

    //这个在什么场景用？
    let _nil = Nil;
    let pair = Pair(1, 2.0);
    println!("{}", pair.1);
    let Pair(integer, decimal) = pair;
    //这里用debug输出，没看出啥特色
    println!("{:?}, {:?}", integer, decimal);
}
