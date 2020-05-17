enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
}

fn main() {
    let pair = (0, -2);
    match pair {
        (0, y) => println!("0, y:{}", y),
        (x, 0) => println!("x:{}, 0", x),
        _ => println!("xxx"),
    }

    let color = Color::RGB(1, 0, 3);
    //haskell的感觉
    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::RGB(0, g, b) => println!("g:{}, b:{}", g, b),
        Color::RGB(r, 0, b) => println!("r:{}, b:{}", r, b),
        Color::RGB(r, g, b) => println!("red:{}, g:{}, b:{}", r, g, b),
    }

    //似乎不能这样做
    // let num = 123;
    // match num {
    //     val => println!("{} is a value", val),
    //     &val => println!("got ref from {}", val),
    // }

    match pair {
        (x, y) if x == y => println!("equal"),
        (x, y) if x + y == 0 => println!("sum to zero"),
        (x @ 2, y @ 3) => println!("(2, 3)"),
        (0, y) => println!("x is zero"),
        _ => println!("others"),
    }
}
