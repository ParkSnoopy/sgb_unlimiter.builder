use std::{
    fs::{ remove_dir_all },
    env::{ current_exe },
    process::{ Command, Stdio },
};
use self_replace;


fn main() {
    let exe = current_exe().unwrap();
    let cwd = exe.as_path().parent().unwrap();
    let comps = cwd.join("comp");
    let _result = Command::new( comps.join( "c01" ) )
        .stdout(Stdio::null())
        .output()
        .expect("failed to execute pre-runner");
    let _result = Command::new( comps.join( "c02" ) )
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
        .expect("failed to execute post-runner");
    let _result = remove_dir_all ( comps );
    let _result = self_replace::self_delete();
}
