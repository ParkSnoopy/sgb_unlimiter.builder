mod localutils;
mod target_builder;
mod proc_info;
mod proc_suspender;

use localutils::printer::{ success, info, warn, debug };
use target_builder::{ build_targets, santinize };
use proc_info::ProcessInformationIterator;
use proc_suspender::suspend_process;

use console::Term;


mod privilege_checker;
use deelevate::{spawn_with_normal_privileges, spawn_with_elevated_privileges};

fn main() {
    let _enabled = ansi_term::enable_ansi_support(); // prelude imported by cargo `colored`


    debug("============== Checking Privilege Level ==============", None);
    privilege_checker::dee_check();
    privilege_checker::plv_check();
    let _ = spawn_with_elevated_privileges();
    debug("=========== spawn_with_elevated_privileges ===========", None);
    privilege_checker::dee_check();
    privilege_checker::plv_check();
    /*let _ = spawn_with_normal_privileges();
    debug("============ spawn_with_normal_privileges ============", None);
    privilege_checker::dee_check();
    privilege_checker::plv_check();*/
    debug("======================================================", None);



    let mut target_vec = build_targets();
    target_vec.push("eraser".to_string());
    debug(format!("Build Target Vector = {:?}", &target_vec).as_str(), None);

    let mut suspended: Vec<String> = Vec::new();
    let mut tried = 0;
    let mut total = 0;

    println!();
    info( "Start scan", None );
    let proc_iterator = ProcessInformationIterator::new();
    println!();
    for proc in proc_iterator {
        // debug(format!("Handle: {}", &proc.name).as_str(), None);
        let procname = santinize( &proc.name );

        if target_vec.contains( &procname ) {
            let (cnt, err_v) = suspend_process(proc.pid);
            debug( format!("Found: {} ( pid {} )", &procname, proc.pid).as_str(), Some(format!("( cnt={cnt}, err={err_v:?} )").as_str()) );
            tried += 1;
            if err_v.iter().map(|b| *b as u32).sum::<u32>() == 0 {
                suspended.push(proc.name.clone());
            }
        }
        total += 1;
    }

    debug("",None);
    success( format!("Done! [ {} / {} ]", suspended.len(), total).as_str(), Some(format!("( with {tried} attempts )").as_str()) );

    suspended.dedup();
    if suspended.len() < 4 {
        warn( format!("{} unique process killed, some process may not killed", suspended.len()).as_str(), None );
    }

    println!();
    info( "Press any key to exit...", None );
    let stdout = Term::buffered_stdout();

    'halt: loop {
        if let Ok(_) = stdout.read_char() {
            break 'halt
        }
    }
}
