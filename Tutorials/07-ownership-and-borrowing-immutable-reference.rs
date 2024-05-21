fn make_string_not_dangle() -> String {
    let s: String = String::from("not dangle");
    s
    // String::from("not dangle"); will be returned as this syntax too because when we return a value in rust, we can do so without adding colon.
}

fn main() {

    // Works
    let x: i32 = 50;
    let y = x;
    println!("{}", x);
    println!("{}", y);

    // Will not work because the value of s has been moved to t.
    // let s: String = String::from("Hello");
    // let t: String = s;
    // println!("{}", s);

    // Works, not recommended though
    let s: String = String::from("Hello");
    let t: String = s.clone();
    println!("{}", t);

    // Works and recommended.
    let s: String = String::from("Hello2");
    let t: &String = &s;
    println!("{}", t);

    // Works
    let r = make_string_not_dangle();
    println!("{}", r);
}