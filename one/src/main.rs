
enum OptionalInt {
    Value(i32),
    Missing,
}

fn main() {
    let x = OptionalInt::Value(5);

    match x {
        OptionalInt::Value(a) if a > 10 => println!("{}", a),
        OptionalInt::Value(a) => println!("{}", a+1),
        OptionalInt::Missing => println!("{}", "missing"),
    }
}
