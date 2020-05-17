use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    let decimal = 65.1114514f32;
    //不允许隐式转换
    //let integer: i32 = decimal;
    let integer = decimal as u8;
    //只允许u8转成char
    let character = integer as char;
    println!("Casting:{}->{}->{}", decimal, integer, character);

    let s1 = "hello";
    let s2 = String::from(s1);
    let num = Number::from(666);
    println!("num:{:?}", num);
    let int = 5;
    let num2: Number = int.into();
    println!("num2:{:?}", num2);

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(3), Err(()));
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 3i32.try_into();
    assert_eq!(result, Err(()));

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("sum:{}", sum);
}
