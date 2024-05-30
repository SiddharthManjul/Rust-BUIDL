#[cfg(test)]

mod tests {

    use ai_func_proc_macro::function_to_string;

    #[allow(dead_code)]
    const OUTPUT: &str = "YO ";

    #[function_to_string]
    fn some_function_for_ai_llm(_whatever_param: &str) {
        /// This is an awesome function
        /// We shall give it to an AI to guess and output
        /// In a structured manner
        println!("{}", OUTPUT);
    }

    #[test]
    fn tests_proc_macro() {
        let x: &str = some_function_for_ai_llm("Some Large Prompt");
        dbg!(x);
    }
}
