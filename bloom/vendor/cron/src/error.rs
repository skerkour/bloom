error_chain! {
    errors {
        Expression(s: String) {
            description("invalid expression")
            display("invalid expression: {}", s)
        }
    }
}
