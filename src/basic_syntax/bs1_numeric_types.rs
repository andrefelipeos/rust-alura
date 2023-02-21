use std::mem::size_of_val;

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    /* Integers */
    let x: i8 = 127; // signed integer
    let y: u8 = 255; // unsigned integer
    println!("x: {}, type: {}, size: {} byte(s)", x, type_of(x), size_of_val(&x));
    println!("y: {}, type: {}, size: {} byte(s)", y, type_of(y), size_of_val(&y));

    let z = 2023; // type inference - 32 bits signed integer
    println!("z: {}, type: {}, size: {} byte(s)", z, type_of(z), size_of_val(&z));


    /* Floats */
    let pi = 3.14159; // type inference - 64 bits float
    println!("pi: {:.2}, type: {}, size: {} byte(s)", pi, type_of(pi), size_of_val(&pi));

    let (mut height, mut weight, mut bmi): (f32, f32, f32); // 32 bits mutable floats
    height = 1.8;
    weight = 80.2;
    bmi = weight / (height);
    bmi = (bmi * 100.0).round() / 100.0;
    println!("bmi: {}, type: {}, size: {} byte(s)", bmi, type_of(bmi), size_of_val(&bmi));
    height = 1.61;
    weight = 62.320;
    bmi = weight / (height);
    bmi = (bmi * 100.0).round() / 100.0;
    println!("bmi: {}, type: {}, size: {} byte(s)", bmi, type_of(bmi), size_of_val(&bmi));
}
