use colored::Colorize;

use crate::config::DEBUG;


pub fn success<S: Into<String> + Colorize>(msg: S, more: Option<S>) {
    print!("{head} {body}",
        head="[+]".bright_green(),
        body=msg.truecolor(144, 238, 144),
    );
    if more.is_some() {
        print!(" {tail}", 
            tail=more.unwrap().truecolor(200, 200, 200)
        );
    };
    println!();
}

pub fn error<S: Into<String> + Colorize>(msg: S, more: Option<S>) {
    print!("{head} {body}",
        head="[x]".red(),
        body=msg.bright_red(),
    );
    if more.is_some() {
        print!(" {tail}", 
            tail=more.unwrap().bright_yellow()
        );
    };
    println!();
}

pub fn warn<S: Into<String> + Colorize>(msg: S, more: Option<S>) {
    print!("{head} {body}",
        head="[-]".yellow(),
        body=msg.bright_red(),
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
        head="[i]".bright_cyan(),
        body=msg.truecolor(173, 216, 230),
    );
    if more.is_some() {
        print!(" {tail}", 
            tail=more.unwrap().bright_blue()
        );
    };
    println!();
}

// Simplified due to so many `.as_str()` calls
pub fn debug(msg: String) {
    if DEBUG {
        println!("{head} {body}",
            head="[#]".yellow(),
            body=msg.as_str().truecolor(200, 200, 160),
        );
    }
}

pub fn debug_v<S: Into<String> + Colorize>(msg: S, more: Option<S>) {
    if DEBUG {
        print!("{head} {body}",
            head="[#]".yellow(),
            body=msg.truecolor(200, 200, 160),
        );
        if more.is_some() {
            print!(" {tail}", 
                tail=more.unwrap().bright_magenta()
            );
        };
        println!();
    }
}

pub fn debug_s<S: Into<String> + Colorize>(msg: S) {
    if DEBUG {
        println!("{head} {body}",
            head="[✓]".yellow(),
            body=msg.truecolor(144, 238, 144),
        );
    }
}

pub fn debug_e<S: Into<String> + Colorize>(msg: S) {
    if DEBUG {
        println!("{head} {body}",
            head="[✕]".yellow(),
            body=msg.bright_red(),
        );
    }
}
