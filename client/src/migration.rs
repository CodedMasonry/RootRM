#[cfg(target_os = "windows")]
use windows;

#[cfg(target_os = "windows")]
pub fn process_injection(pid: &str) {
    let process = windows::Win32::System::Threading::OpenProcess(PROCESS_ACCESS_RIGHTS, false, pid);
}

#[cfg(target_os = "linux")]
pub fn process_injection(pid: &str) {}
