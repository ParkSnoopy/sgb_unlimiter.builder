use colored::Colorize;

const DEBUG: bool = true;


fn success<S: Into<String>>(msg: S, more: Option<S>) -> String {
    todo!();
}

fn error<S: Into<String>>(msg: S, more: Option<S>) -> String {
    todo!();
}

fn info<S: Into<String>>(msg: S, more: Option<S>) -> String {
    todo!();
}

fn debug<S: Into<String>>(msg: S, more: Option<S>) -> Option<String> {
    if !DEBUG { return None }
    todo!();
}
