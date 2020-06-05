//被借用者的声明周期，必须比借用者的生命周期长
//带生命周期参数，却没有指定其值，默认为static

//不带生命周期参数的函数，输入的引用生命周期怎么算？
fn print_one(val: &i32) {
    println!("val:{}", val);
}

fn print_one_with_lifetime<'a>(val: &'a i32) {
    println!("val is {}", val);
}

#[derive(Debug)]
struct BorrowTwo<'a> {
    x: &'a i32,
    y: &'a i32,
}

static NUM: i32 = 42;

fn get_num<'a>(_: &'a i32) -> &'a i32 {
    return &NUM;
}

fn main() {
    let x = 7;
    print_one(&x);
    print_one_with_lifetime(&x);
    {
        let y = 8;
        //这种情况下，'a是y的生命周期？
        let bt = BorrowTwo{x:&x, y:&y};
        println!("BorrowTwo:{:?}", bt);
    }
    {
        let z = 9;
        //强制把NUM的生命周期降成z的生命周期
        let the_num = get_num(&z);
        println!("the_num:{}", the_num);
    }
    println!("NUM:{}", NUM);
}