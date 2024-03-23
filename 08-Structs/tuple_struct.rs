struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(10,20,50);
    let origin = Point(30,40,60);

    let a = black.0;
    let b = black.1;
    let c = black.2;

    let x = origin.0;

    println!("{} {} {}", a,b,c);
    println!("{}",x);
}