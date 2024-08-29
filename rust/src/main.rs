// Config Vars
pub mod config;
pub mod error;
pub mod printer;

mod state;
mod decode;
mod privilige;
mod process;
// mod cleaner; /* deprecated @ v1.1.5 */

use crate::printer::{ info, debug, debug_s, debug_e, error as error_printer };
use crate::process::{ process_iter, get_process_handle, suspend_process_handle, is_target_process, santinize };

use std::time::{ Duration };
use std::thread::sleep;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::{ Arc };

use eyre::Result;
use ansi_escapes;
use ctrlc;


fn main() -> Result<()> {
    let running = Arc::new( AtomicBool::new(true) );
    let r = running.clone();
    ctrlc::set_handler( move || {
        r.store( false, Ordering::SeqCst );
    }).expect("Failed to bind handler on `Ctrl+C`");

    // Initialize Jobs : Set Up
    init();

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
    prepare_run();
    let estimated_runs = config::SUSPEND_UNTIL / config::SUSPEND_EACH;
    for iteration in 1..=estimated_runs {

        if !running.load( Ordering::SeqCst ) {
            println!();
            error_printer("Got `Ctrl+C` signal!", None);
            info("Performing early exit...", None);

            sleep(Duration::new( config::CTRLC_IDLE, 0 ));
            break;
        }

        prepare_iteration();

        info(format!("Estimated {}", &estimated_runs).as_str(), None);
        info(format!("Iteration {}", &iteration).as_str(),
        Some(format!("( {:2.2} % )", 
            (100_f64 * iteration as f64 / estimated_runs as f64)
        ).as_str()));

        let suspend_state = do_suspend_targets(&targets);
        let was_success: bool = suspend_state.is_successful_run();

        debug(format!("WAS_SUCCESSFUL_RUN: {:?}", was_success));
        debug(format!("ITER {:03}", &iteration));
        debug("".to_string());

        sleep(Duration::new( config::SUSPEND_EACH.into(), 0 ));
    }


    // POST-RUN IDLE LOOP //
    if running.load( Ordering::SeqCst ) {
        println!();
        info(format!("This window will automatically close after {} second(s)", config::IDLE_AFTER_FINISH).as_str(), None);
        info("You can close this window manually", None);
        sleep(Duration::new( config::IDLE_AFTER_FINISH, 0 ));

        /*
        use std::process::Command;
        Command::new("cmd")
            .arg("/c")
            .arg("pause")
            .status()
            .unwrap();
        */
    }

    Ok(())
}


fn init() {
    privilige::elevate();
    let _enabled = ansi_term::enable_ansi_support();
}

fn prepare_run() {
    print!("\n\n{}",
        ansi_escapes::CursorSavePosition,
    );
}

fn prepare_iteration() {
    print!("{}\n\n{}",
        ansi_escapes::ClearScreen,
        ansi_escapes::CursorRestorePosition,
    );
}


fn do_suspend_targets(targets: &Vec<String>) -> state::SuspendState {
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
