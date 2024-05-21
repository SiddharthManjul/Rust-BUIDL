// Static
const MSG_CONST: &str = "hello_constant";

fn main() {

    // Heap
    let s: String = String::from("Hello String");
    let s2 = &s[0..5];
    println!("{}", s2);

    //Static: This is a String Literal and it's stored statically and msg is pointing to the literal thus msg is in stack and the data is Static.
    let msg: &str = "Hello2";
    println!("{}", msg);

    // Heap
    let msg_string: String = "Hello3".to_string();
    println!("{}", msg_string);

    println!("{}", MSG_CONST);
}