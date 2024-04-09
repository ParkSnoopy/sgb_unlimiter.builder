use winapi::shared::minwindef::FALSE;
use winapi::shared::ntdef::HANDLE;

use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{ 
    OpenThread, 
    SuspendThread, 
};
use winapi::um::tlhelp32::{ 
    CreateToolhelp32Snapshot, 
    TH32CS_SNAPTHREAD, 
    Thread32First, 
    Thread32Next, 
    THREADENTRY32, 
};
use winapi::um::winnt::THREAD_SUSPEND_RESUME;


pub fn suspend_process(pid: u32) -> (u32, Vec<bool>) {
    unsafe {
        let mut errors: Vec<bool> = Vec::new();
        let mut count : u32 = 0;

        let te: &mut THREADENTRY32 = &mut std::mem::zeroed();
        (*te).dwSize = std::mem::size_of::<THREADENTRY32>() as u32;

        let snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

        if Thread32First(snapshot, te) == 1 {
            loop {
                if pid == (*te).th32OwnerProcessID {
                    let tid = (*te).th32ThreadID;

                    let thread: HANDLE = OpenThread(THREAD_SUSPEND_RESUME, FALSE, tid);
                    errors.push( SuspendThread(thread) as i32 == -1i32 );

                    CloseHandle(thread);
                    count += 1;
                }

                if Thread32Next(snapshot, te) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);

        (count, errors)
    }
}

