fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} from r1 and {} form r2", r1, r2);

    let r3 = &mut s;
    println!("{} from r3", r3);
}