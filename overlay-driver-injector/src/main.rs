use std::os::windows::prelude::FromRawHandle;
use winapi::{
    shared::minwindef::FALSE,
    um::{
        processthreadsapi::OpenProcess,
        winnt::{
            PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION,
            PROCESS_VM_READ, PROCESS_VM_WRITE,
        },
    },
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let overlay_driver_dll_file_path = args[1].clone();
    let target_process_pid = args[2].parse::<u32>().unwrap();

    unsafe {
        let target_process_handle = OpenProcess(
            PROCESS_CREATE_THREAD
                | PROCESS_QUERY_INFORMATION
                | PROCESS_VM_OPERATION
                | PROCESS_VM_WRITE
                | PROCESS_VM_READ,
            FALSE,
            target_process_pid,
        );

        if target_process_handle.is_null() {
            panic!();
        }

        dll_syringe::Syringe::for_process(dll_syringe::process::OwnedProcess::from_raw_handle(
            target_process_handle as _,
        ))
        .inject(overlay_driver_dll_file_path)
        .unwrap();
    }
}
