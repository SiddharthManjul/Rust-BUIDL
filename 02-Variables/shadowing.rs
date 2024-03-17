fn main() {
    let x : i32 = 5;
    {
        let x : i32 = 12;
        assert_eq!(x, 12); // x=12 is used here.
    }
    assert_eq!(x, 5); // x=5 is used here.

    let x : i32 = 42;
    assert_eq!(x, 42); // x=42 is used here and shadowed x=5 making the current value 42.

    let x : i32 = 50;
    println!("{}", x); // x=50 is used here and shadowed x=42 making the current value 50.
}