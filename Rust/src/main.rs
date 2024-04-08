mod printer;
use printer::{ success, error, info, warn, debug };

mod decoder;

mod proc_info;
use proc_info::ProcessInformationIterator;

mod proc_suspender;
use proc_suspender::suspend_process;

use console::Term;
use ansi_term;


fn build_targets() -> Vec<String> {
    info( "Building target vector from pre-prepared bytes...", None );

    let bytes = decoder::decode("EHQ&*E--2@:2OENF*)G@G@b5lB4Yt&:2OENF*)G@G@b6)G&g>qBP)-n@sWH<:2XZ^GA;AJAn1");

    if bytes.is_err() {
        error( "Decoder decode failed", None );
        panic!();
    }

    info( "Decode success!", None );

    String::from_utf8(bytes.unwrap()).unwrap().split("N").map(str::to_string).collect()
}

fn rm_ext_lower(pname: &String) -> String {
    pname.split(".").collect::<Vec<&str>>()[0].to_lowercase()
}

fn main() {
    let _enabled = ansi_term::enable_ansi_support();

    let target_vec = build_targets();

    let mut unique: Vec<String> = Vec::new();
    let mut suspended = 0;
    let mut total     = 0;

    info( "Scanning...", None );
    for proc in ProcessInformationIterator::new() {
        let procname = rm_ext_lower(&proc.name);

        if target_vec.contains( &procname ) {
            let (c, e) = suspend_process(proc.pid);
            debug( format!("Found: {} ( pid {} )", procname, proc.pid).as_str(), Some(format!("( c={c}, e={e} )").as_str()) );
            suspended += 1;
            unique.push(proc.name.clone());
        }
        total += 1;
    }
    success( "Done!", Some(format!("[ {suspended} / {total} ]").as_str()) );

    unique.dedup();
    if unique.len() < 4 {
        warn( format!("{} unique process killed, some process may not killed", unique.len()).as_str(), None );
    }

    info( "Press any key to exit...", None );
    let stdout = Term::buffered_stdout();

    'halt: loop {
        if let Ok(_) = stdout.read_char() {
            break 'halt
        }
    }
}
