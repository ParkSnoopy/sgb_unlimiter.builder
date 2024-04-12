#![allow(dead_code)]

// my localutils
mod localutils;
use localutils::printer::{ success, info, warn, debug, error };
mod target;

// for: at the end, press any key to exit
use console::Term;

// get pids
mod proc_info;
use proc_info::ProcessInformationIterator;

// WINDOWS NT (PowerShell) utility
mod windows_native_local;
use windows_native_local::ntpsapi::{ NtSuspendProcess };
use windows::Win32::{
    System::Threading::{ OpenProcess, PROCESS_ALL_ACCESS, PROCESS_SUSPEND_RESUME },
    Foundation::{ HANDLE, NTSTATUS },
};

/*use ntutils::{
    OpenProcess,
    NtSuspendProcess,
    NtResumeProcess,
    PROCESS_ALL_ACCESS,
};*/

fn main() {
    let _enabled = ansi_term::enable_ansi_support(); // prelude imported by cargo `colored`

    let target_vec = target::build();
    // let mut target_vec = build_targets();
    // target_vec.push("eraser".to_string());
    debug(format!("Build Target Vector = {:?}", &target_vec).as_str(), None);

    let mut suspended: Vec<NTSTATUS> = Vec::new();
    let mut tried: u32 = 0;
    let mut total: u32 = 0;

    println!();
    info( "Start scan", None );
    for proc in ProcessInformationIterator::new() {
        // debug(format!("Handle: {}", &proc.name).as_str(), None);
        let procname = target::santinize( &proc.name );

        if target_vec.contains( &procname ) {
            debug( format!("Found: {} ( pid {} )", &procname, proc.pid).as_str(), None);//Some(format!("( cnt={cnt}, err={err_v:?} )").as_str()) );

            let handle_result = unsafe { OpenProcess(PROCESS_SUSPEND_RESUME, false, proc.pid) };
            let mut suspend_ret_value: NTSTATUS = Default::default();

            match handle_result {
                Ok(handle) => {
                    suspend_ret_value = unsafe { NtSuspendProcess(handle) };
                },
                Err(err) => {
                    error(format!("Get Handle Failed: {}", err.message()).as_str(), None);
                }
            }

            tried += 1;
            suspended.push(suspend_ret_value);

        }
        total += 1;
    }

    debug("",None);
    success( format!("Done! [ {suspended:?} / {total} ]").as_str(), Some(format!("( with {tried} attempts )").as_str()) );

    if suspended.len() < 4 {
        warn( format!("{suspended:?} unique process killed, some process may not killed").as_str(), None );
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
