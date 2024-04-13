use std::{
    fs::{ remove_file },
    env::{ current_exe },
};
use self_replace;
use crate::config::EXTERNAL_SUSPEND_EXE_NAME;


pub fn clean_self() {
    let _result = remove_file( current_exe().unwrap().as_path().parent().unwrap().join( EXTERNAL_SUSPEND_EXE_NAME ) );
    let _result = self_replace::self_delete();
}
