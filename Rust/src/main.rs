#![allow(dead_code)]

// my localutils
mod localutils;
use localutils::printer::{ success, info, warn, debug };
mod target;

// for: at the end, press any key to exit
use console::Term;

// get pids
mod proc_info;
use proc_info::ProcessInformationIterator;

// PsTools ( external .exe binary dep )
mod external_suspend;


fn main() {
    let _enabled = ansi_term::enable_ansi_support(); // prelude imported by cargo `colored`

    let target_vec = target::build();
    //let mut target_vec = target::build();
    //target_vec.push("lqndauccd".to_string());
    //target_vec.push("igfxcuiservice".to_string());

    debug(format!("Built Target Vector = {:?}", &target_vec).as_str(), None);

    let mut suspended: u32 = 0;
    let mut tried    : u32 = 0;
    let mut total    : u32 = 0;

    println!();
    info( "Start scan", None );
    for proc in ProcessInformationIterator::new() {
        //debug(format!("Handle: {}", &proc.name).as_str(), None);
        let procname = target::santinize( &proc.name );

        if target_vec.contains( &procname ) {
            tried += 1;
            debug("", None);
            debug( format!("Found: {} ( pid {} )", &procname, proc.pid).as_str(), None);//Some(format!("( cnt={cnt}, err={err_v:?} )").as_str()) );

            let output = external_suspend::run_external_suspend(proc.pid);
            let log = external_suspend::filter_stdout( String::from_utf8(output.stdout.clone()).unwrap() );

            if log.starts_with("Process") {
                suspended += 1;
            }
            debug( log.as_str(), None );

        }
        total += 1;
    }

    println!();
    success( format!("Done! [ {suspended} / {total} ]").as_str(), Some(format!("( with {tried} attempts )").as_str()) );

    if suspended < 4 {
        warn( format!("Only {suspended} unique process handled, some process may not handled").as_str(), None );
    }


    // POST-RUN IDLE LOOP //
    println!();
    info( "Press any key to exit...", None );
    let stdout = Term::buffered_stdout();
    'halt: loop {
        if let Ok(_) = stdout.read_char() {
            break 'halt
        }
    }

}
