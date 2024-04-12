use std::{
	os::windows::process::{ ExitStatusExt },
	env::{ current_exe },
	process::{ Command, Output, ExitStatus },
};
use crate::localutils::printer::{ debug, error };

const EXECUTABLE_NAME: &'static str = "pkg";


pub fn run_external_suspend(pid: u32) -> Output {
	let target = current_exe().unwrap().as_path().parent().unwrap().join( EXECUTABLE_NAME );
	debug( format!("Executable at : {}", target.display()).as_str(), None );
    let result = Command::new(target)
        .arg(pid.to_string())
        .output();

    match result {
    	Ok(output) => { output },
    	Err(err) => { error(format!("{}",err.to_string()).as_str(), None); Output{ status: ExitStatus::from_raw(1), stdout: "".into(), stderr: err.to_string().into()} }
    }
}

pub fn filter_stdout(msg: String) -> String {
	// "\r\nPsSuspend v1.08 - Process Suspender\r\nCopyright (C) 2001-2023 Mark Russinovich\r\nSysinternals\r\n\r\nProcess 8172 suspended.\r\n\r\n"
	let mut filtered = String::new();

	for line in msg.lines() {
		filtered = match line {
			"" => { filtered },
			_  => { line.to_string() },
		}
	}

	filtered
}
