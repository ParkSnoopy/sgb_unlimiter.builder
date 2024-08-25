use winapi::um::processthreadsapi::{ OpenProcess };
use winapi::um::errhandlingapi::{ GetLastError };
use winapi::um::winnt::{ HANDLE, PROCESS_ALL_ACCESS };
use winapi::shared::ntdef::{ NULL, NTSTATUS };
use winapi::shared::minwindef::{ DWORD };
use ntapi::ntpsapi::{ NtSuspendProcess };

use tasklist::{ Tasklist, Process };

use crate::config::{ SUSPEND_ATTEMPT };
use crate::printer::{ debug };


pub fn process_iter() -> impl Iterator<Item=Process> {
    unsafe{
        Tasklist::new()
        // println!( "{} {} {}", i.get_pid(), i.get_pname(), i.get_user() );
    }
}

pub fn get_process_handle(pid: u32) -> Result<HANDLE, DWORD> {
    let proc_handle = unsafe { OpenProcess( PROCESS_ALL_ACCESS, 0, pid ) };
    match proc_handle {
        NULL => { Err( unsafe { GetLastError() } ) },
        _    => { Ok(proc_handle) }
    }
}

pub fn suspend_process_handle(proc_handle: HANDLE) -> bool {
    let suspend_result: NTSTATUS = unsafe {
        let result = NtSuspendProcess( proc_handle );
        for _ in 1..SUSPEND_ATTEMPT { NtSuspendProcess( proc_handle ); }
        result
    };

    debug(format!("Result = {}", suspend_result).as_str(), None);
    debug(format!("LError = {}", unsafe{ GetLastError() }).as_str(), None);

    match suspend_result {
        0 => { true },
        _ => {
            match unsafe{ GetLastError() } {
                0 => { true  },
                _ => { false },
            }
        },
    }
}

pub fn is_target_process(targets: &Vec<String>, proc_name: &String) -> bool {
    for target in targets.iter() {
        if proc_name.contains( target ) {
            return true;
        }
    }
    false
}

pub fn santinize(proc: &Process) -> String {
    proc.get_pname().to_lowercase()
}
