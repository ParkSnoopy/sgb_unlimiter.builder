use colored::Colorize;

const DEBUG: bool = true;


/*
fn _colored_printer<S: Into<String>>(head, head_color, body, body_color, tail, tail_color) {
    unimplemented!();
}
*/


pub fn success<S: Into<String> + Colorize>(msg: S, more: Option<S>) {
    print!("{head} {body}",
        head="[ ✓ ]".bright_green(),
        body=msg.white(),
    );
    if more.is_some() {
        print!(" {tail}", 
            tail=more.unwrap().bright_black()
        );
    };
    println!();
}

pub fn error<S: Into<String> + Colorize>(msg: S, more: Option<S>) {
    print!("{head} {body}",
        head="[ ✕ ]".red(),
        body=msg.yellow(),
    );
    if more.is_some() {
        print!(" {tail}", 
            tail=more.unwrap().bright_yellow()
        );
    };
    println!();
}

pub fn info<S: Into<String> + Colorize>(msg: S, more: Option<S>) {
    print!("{head} {body}",
        head="[ * ]".bright_cyan(),
        body=msg.white(),
    );
    if more.is_some() {
        print!(" {tail}", 
            tail=more.unwrap().bright_blue()
        );
    };
    println!();
}

pub fn debug<S: Into<String> + Colorize>(msg: S, more: Option<S>) {
    if DEBUG {
        print!("{head} {body}",
            head="[ ∴ ]".yellow(),
            body=msg.bright_purple(),
        );
        if more.is_some() {
            print!(" {tail}", 
                tail=more.unwrap().bright_magenta()
            );
        };
        println!();
    }
}
