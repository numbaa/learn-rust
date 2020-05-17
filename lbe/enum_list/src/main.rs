use List::*;    //这个List是下面那个List

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    //不是&self，而且发生了所有权转移？
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    //惊了
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => "Nil".to_owned(),
        }
    }
}


fn main() {
    //pei，假List
    let mut list = List::new();

    list = list.prepend(2);
    list = list.prepend(4);
    list = list.prepend(6);
    println!("list len:{}, content:{}", list.len(), list.stringify());
}
