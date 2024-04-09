use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenThread;
use winapi::um::processthreadsapi::ResumeThread;
use winapi::um::processthreadsapi::SuspendThread;
use winapi::um::tlhelp32::CreateToolhelp32Snapshot;
use winapi::um::tlhelp32::Process32FirstW;
use winapi::um::tlhelp32::Process32NextW;
use winapi::um::tlhelp32::PROCESSENTRY32W;
use winapi::um::tlhelp32::TH32CS_SNAPPROCESS;
use winapi::um::tlhelp32::TH32CS_SNAPTHREAD;
use winapi::um::tlhelp32::Thread32First;
use winapi::um::tlhelp32::Thread32Next;
use winapi::um::tlhelp32::THREADENTRY32;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::THREAD_SUSPEND_RESUME;


fn find_process_id_by_name(name: &str) -> u32 {
    unsafe {
        let pe: &mut PROCESSENTRY32W = &mut std::mem::zeroed();
        (*pe).dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        let snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        if Process32FirstW(snapshot, pe) == 1 {
            loop {
                let process_name = String::from_utf16_lossy(&pe.szExeFile);
                let process_name = process_name.trim_end_matches(char::from(0));

                if process_name == name {
                    break;
                }

                if Process32NextW(snapshot, pe) == 0 {
                    (*pe).th32ProcessID = 0;
                    break;
                }
            }
        }

        CloseHandle(snapshot);

        (*pe).th32ProcessID
    }
}

fn suspend_process(pid: u32) -> (u32, bool) {
    unsafe {
        let mut has_err = false;
        let mut count: u32 = 0;

        let te: &mut THREADENTRY32 = &mut std::mem::zeroed();
        (*te).dwSize = std::mem::size_of::<THREADENTRY32>() as u32;

        let snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

        if Thread32First(snapshot, te) == 1 {
            loop {
                if pid == (*te).th32OwnerProcessID {
                    let tid = (*te).th32ThreadID;

                    let thread: HANDLE = OpenThread(THREAD_SUSPEND_RESUME, FALSE, tid);
                    has_err |= SuspendThread(thread) as i32 == -1i32;

                    CloseHandle(thread);
                    count += 1;
                }

                if Thread32Next(snapshot, te) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);

        (count, has_err)
    }
}

fn resume_process(pid: u32) -> (u32, bool) {
    unsafe {
        let mut has_err = false;
        let mut count: u32 = 0;

        let te: &mut THREADENTRY32 = &mut std::mem::zeroed();
        (*te).dwSize = std::mem::size_of::<THREADENTRY32>() as u32;

        let snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

        if Thread32First(snapshot, te) == 1 {
            loop {
                if pid == (*te).th32OwnerProcessID {
                    let tid = (*te).th32ThreadID;

                    let thread: HANDLE = OpenThread(THREAD_SUSPEND_RESUME, FALSE, tid);
                    has_err |= ResumeThread(thread) as i32 == -1i32;

                    CloseHandle(thread);
                    count += 1;
                }

                if Thread32Next(snapshot, te) == 0 {
                    break;
                }
            }
        }

        CloseHandle(snapshot);

        (count, has_err)
    }
}

fn main() {

    println!("\n-----------------------");
    println!("PHASE 1: FIND PROCESS ID");
    println!("-----------------------");
    let pid = 2428;/* find_process_id_by_name(name);

    match pid {
        0 => {
            println!("Couldn't find PID for process {}.", name);
            return;
        }
        _ => println!("Found PID {} for process {}.", pid, name),
    }*/


    println!("\n-----------------------");
    println!("PHASE 2: SUSPEND PROCESS");
    println!("-----------------------");
    let (count, failed) = suspend_process(pid);
    if failed {
        print!("Failed to suspend process");
        return;
    }
    for i in (1..=8).rev() {
        print!("Suspending for {} second{}", i, if i == 1 { "" } else { "s" });
        for _ in 1..=3 {
            print!("{}", ".");
            std::io::stdout().flush().unwrap();
            sleep(Duration::from_millis(333))
        }
        print!("{}", "\r");
    }
    println!("Suspended {} threads. Done.", count);


    println!("\n-----------------------");
    println!("PHASE 3: RESUME PROCESS");
    println!("-----------------------");
    let (count, failed) = resume_process(pid);
    if failed {
        print!("Failed to suspend process");
        return;
    }
    println!("Resumed {} threads. Done.", count);
}
