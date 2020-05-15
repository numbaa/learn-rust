
#[derive(Debug)]
struct Point3D(f32, f32, f32);

fn main() {
    let tuple_element = (1u8, 2i8, 3u16, 4i16, 5u32, 6i32, 7u64, 8i64, 0.1f32, 0.2f64, 'x', false);
    println!("{:?}", tuple_element);
    println!("{}, {}", tuple_element.0, tuple_element.4);

    let tao_wa = ((5, true), ('a', 5, 4.14));
    println!("{:?}", tao_wa);
    println!("single element: {:?}, an integer:{}", (123,), 123);
    let point = Point3D(3.2, 5.6, -7.8);
    println!("struct:{:?}", point);
}
