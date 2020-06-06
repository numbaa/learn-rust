struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci{ curr:1, next:1}
}

fn main() {
    //
    let mut sequence = 0..3; //不包括3
    //不需要begin什么的，难道sequence本身就是一个iterator?
    println!("> {:?}", sequence.next()); //Some(0)
    println!("> {:?}", sequence.next()); //Some(1)
    println!("> {:?}", sequence.next()); //Some(2)
    println!("> {:?}", sequence.next()); //None


    //for是一个语法糖，它遍历Iterator，直到返回None
    for i in 0..3 {
        println!("> {}", i);
    }

    for i in fibonacci().skip(4).take(3) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];
    for i in array.iter() {
        println!("> {}", i);
    }
}
