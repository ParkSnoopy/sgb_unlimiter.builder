// Config Vars
pub mod config;
pub mod error;
pub mod printer;

mod state;
mod decode;
mod privilige;
mod process;
// mod cleaner; /* deprecated @ v1.1.5 */

use crate::printer::{ info, debug, debug_s, debug_e };
use crate::process::{ process_iter, get_process_handle, suspend_process_handle, is_target_process, santinize };

use std::time::{ Duration };
use std::thread::sleep;

use eyre::Result;


fn main() -> Result<()> {
    // Initialize Jobs : Set Up
    {
        let _ = ansi_term::enable_ansi_support();
        privilige::elevate();
    }

    // Initialize Jobs : Build Target Vector
    let targets = if !config::DEBUG {
        decode::get_prebuilt()
    } else {
        let mut targets = decode::get_prebuilt();
        targets.push("eraser".to_string());
        targets
    };
    debug(format!("Built Target Vector = {:?}", &targets));


    // Main Termination Loop
    let mut successful_runs: u32 = 0;
    for iteration in 1..=( config::SUSPEND_UNTIL / config::SUSPEND_EACH ) {
        if do_suspend_targets(&targets) {
            successful_runs += 1;
        }
        if successful_runs > config::EARLY_TERMINATE_THRESHOLD {
            info("Early termination due to consistent successful runs", None);
            break;
        }

        debug(format!("ITER {:03}", &iteration));
        debug(format!("SRUN {:03}", &successful_runs));
        debug("".to_string());

        sleep(Duration::new( config::SUSPEND_EACH.into(), 0 ));
    }


    // POST-RUN IDLE LOOP //
    {
        println!();
        use std::process::Command;
        Command::new("cmd")
            .arg("/c")
            .arg("pause")
            .status()
            .unwrap();
    }

    Ok(())
}

fn do_suspend_targets(targets: &Vec<String>) -> bool {
    println!();
    info( "Start scan", None );
    debug("".to_string());

    let mut suspend_state = state::SuspendState::new();

    for proc in process_iter() {
        // - HELP
        // process_id   : proc.get_pid()
        // process_name : proc.get_pname()
        // process_user : proc.get_user()
        // 
        // debug(format!("Process - {} // {} // {}", proc.get_pid(), proc.get_pname(), proc.get_user()).as_str(), None);

        let proc_name = santinize( &proc );

        if is_target_process(targets, &proc_name) {

            debug( format!("PName={}", &proc_name) );

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

    // print: current suspend iteration's result status
    suspend_state.display();

    // return: suspended process count exceeded `config::SUSPEND_SHOULD` threshold or not
    suspend_state
}
