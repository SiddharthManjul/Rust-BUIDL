// MACRO CAPTURES

/* expr
    matches to a valid rust expression.
    "hello"to_string, vec![1, 2, 3], 1 + 2, 1
*/

/* stmt
    matches to a rust statement
    let x = 5, x.push(1), return Some(x)
*/

/* ident
    matches to a rust identifier
    variable name, function name, module name
*/

/* ty
    matches to a rust type
    i32, Vec<String>, Option<T>
*/

/* path
    matches to a rust path
    std::collections::HashMap
*/

// Repetition Specifier

/*
   * - match zero or more repititions
   + - match one or more repititions
   ? - match zero or one repititions
*/

/*
    Note: To enable a macro to be used in other files if the crate containing your macro is imported, add this above your macro:

    #[macro_export]

    macro_rules! my_vec { ... }
*/

#[cfg(test)]

mod tests {

    macro_rules! mad_sills {
        // ($x: expr) => {
        //     format!("You sent an expression {}", $x)
        // };
        ($y: ty) => {
            match stringify!($y) {
                "i32" => "You sent an i32 type".to_string(),
                _ => "You sent something else".to_string(),
            }
        };
    }

    macro_rules! local_vec {
        ($($x: expr), +) => {
            
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )+
                temp_vec
            }
        };
    }

    #[test]
    fn tests_declarative_macro() {
        // let some_var: String = mad_sills!(1 + 2);
        // dbg!(some_var);

        let some_type: String = mad_sills!(&str);
        dbg!(some_type);

        let mut y: Vec<i32> = local_vec!(1);
        y.push(2);
        dbg!(y);
    }
}
