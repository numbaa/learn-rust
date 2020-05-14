use std::fmt;

struct Structure(i32);

//笔记：不支持泛型
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug)]
struct Point
{
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}

struct MyList(Vec<i32>);

impl fmt::Display for MyList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = &self.0;
        write!(f, "[")?;
        for (count, item) in v.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

fn main() {
    //似乎不能let p = Point(3.2, 4.5)
    //s.0可以，但是p.0不可以
    let s = Structure(3);
    let p = Point{x:3.2, y:4.5};
    //TODO: 研究下vec!怎么实现的
    let l = MyList(vec![55, 66, 77]);
    //the_struct=s要在最后
    println!("Debug:'{:?}', Display:'{}', list:'{}',  {the_struct}",
        p, p, l, the_struct=s);
}
