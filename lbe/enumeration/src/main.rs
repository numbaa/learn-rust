//Rust的枚举太香了
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x: i64, y:i64}
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click{x, y} => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('j');
    //to_string 和 to_owned啥区别
    let pasted = WebEvent::Paste("the content".to_string());
    let click = WebEvent::Click{x:200, y:300};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    //编译失败
    //println!("value of pageload is {}", WebEvent::PageLoad as i32);
    //as f32 失败
    println!("zero is {}", Number::Zero as i64);
    println!("roses are #{:06x}", Color::Red as i32);
}
