// Config Vars
pub mod config;
pub mod error;
pub mod printer;

mod state;
mod decode;
mod privilige;
mod process;
mod cleaner;

use crate::printer::{ info, error, debug, debug_s, debug_e };
use crate::process::{ process_iter, get_process_handle, suspend_process_handle, is_target_process };


fn main() {
    { // Initialize
        let _ = ansi_term::enable_ansi_support();
        privilige::elevate();
    }

    // let targets = decode::get_prebuilt();
     let mut targets = decode::get_prebuilt();
     targets.push("eraser".to_string());

    debug(format!("Built Target Vector = {:?}", &targets).as_str(), None);

    let mut suspend_state = state::SuspendState::new();

    println!();
    info( "Start scan", None );
    debug("", None);

    for proc in process_iter() {
        // - HELP
        // process_id   : proc.get_pid()
        // process_name : proc.get_pname()
        // process_user : proc.get_user()
        // 
        // debug(format!("Process - {} // {} // {}", proc.get_pid(), proc.get_pname(), proc.get_user()).as_str(), None);

        let proc_name = decode::santinize( &proc );

        if is_target_process(&targets, &proc_name) {

            debug( format!("PName={}", &proc_name).as_str(), None );

            if proc.get_user() == "access denied:OpenProcess failed".to_string() {
                suspend_state.fail_access_denied();
                debug_e( "Error Access Denied" );
                continue;
            }

            let proc_handle_result = get_process_handle( proc.get_pid() );
            if proc_handle_result.is_err() {
                suspend_state.fail_get_handle();
                debug_e( "Error get handle" );
                debug_e( format!("Err: {:?}", proc_handle_result.err().unwrap()).as_str() );
                continue;
            }

            let proc_handle = proc_handle_result.unwrap();
            if !suspend_process_handle( proc_handle ) {
                suspend_state.fail_suspend_process();
                debug_e("Handle Error");
                continue;
            }

            suspend_state.success_suspend_process();
            debug_s("Handle Success");

        } else {
            suspend_state.no_match();
        }
    }

    println!();
    suspend_state.display();

    // POST-RUN IDLE LOOP //
    println!();
    info( "Press ENTER to exit...", None );
    if let Err(err) = std::io::stdin().read_line(&mut String::new()) {
        error( format!("{}", err.to_string()).as_str(), None );
    }

    cleaner::clean_self();
}