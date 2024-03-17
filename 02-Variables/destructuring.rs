fn main() {
    let (x,y);

    (x,..) = (3,2); // 3 is assigned to x and there is noting to do with 2 so we used ..
    [..,y] = [2,1]; // there is nothing to do with 2 so we used .. and 1 is assigned to y

    assert_eq!([x,y], [3,1]);
    println!("Success!");
}