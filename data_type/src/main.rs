fn main() {
    // tuple

    let tup = ("wap", 123, true);

    let (x, y, z) = tup;

    println!("tuple data is x: {x} , y: {y}, z: {z}");
    println!("0th value {}", tup.0);
    println!("1th value {}", tup.1);
    println!("2th value {}", tup.2);
    println!("testing");

    let a = 2;

    let add1 = a as f32 + 3.2;
    let add2 = 2f32 + 3.2;

    println!("add two number add1 : {add1}    add2: {add2}");

    let y = 3.2;
    println!("floating value is {y}");

    let (num, b): (u8, bool) = num_data().overflowing_add(100);
    println!("secret number {num} :: {b}");
}

fn num_data() -> u8 {
    return 200;
}
