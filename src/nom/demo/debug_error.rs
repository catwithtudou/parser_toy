use nom::error::{ContextError, ErrorKind, ParseError};

#[derive(Debug)]
struct DebugError {
    message: String,
}

impl ParseError<&str> for DebugError {
    // 打印出具体错误的解析器类型
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        let message = format!("【{:?}】:\t{:?}\n", kind, input);
        println!("{}", message);
        DebugError { message }
    }

    // 若遇到组合器的多个错误则打印出其他上下文信息
    fn append(input: &str, kind: ErrorKind, other: Self) -> Self {
        let message = format!("【{}{:?}】:\t{:?}\n", other.message, kind, input);
        println!("{}", message);
        DebugError { message }
    }

    // 打印出具体期望的字符
    fn from_char(input: &str, c: char) -> Self {
        let message = format!("【{}】:\t{:?}\n", c, input);
        print!("{}", message);
        DebugError { message }
    }

    fn or(self, other: Self) -> Self {
        let message = format!("{}\tOR\n{}\n", self.message, other.message);
        println!("{}", message);
        DebugError { message }
    }
}

impl ContextError<&str> for DebugError {
    fn add_context(_input: &str, _ctx: &'static str, other: Self) -> Self {
        let message = format!("【{}「{}」】:\t{:?}\n", other.message, _ctx, _input);
        print!("{}", message);
        DebugError { message }
    }
}