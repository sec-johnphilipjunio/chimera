pub fn print_tool_name() {
    let banner = r#"
==========================================================
            NOTE: This is a Beta Version.
==========================================================

   ██████╗██╗  ██╗██╗███╗   ███╗███████╗██████╗  █████╗ 
  ██╔════╝██║  ██║██║████╗ ████║██╔════╝██╔══██╗██╔══██╗
  ██║     ███████║██║██╔████╔██║█████╗  ██████╔╝███████║
  ██║     ██╔══██║██║██║╚██╔╝██║██╔══╝  ██╔══██╗██╔══██║
  ╚██████╗██║  ██║██║██║ ╚═╝ ██║███████╗██║  ██║██║  ██║
   ╚═════╝╚═╝  ╚═╝╚═╝╚═╝     ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝

==========================================================
                    version: 0.1.0
==========================================================
    "#;
    println!("{}", banner);
}

pub fn print_help() {
    let line = "=".repeat(58);
    let sub_line = "-".repeat(58);

    println!("{}", line);
    println!("           CHIMERA INJECTION FRAMEWORK - HELP MENU");
    println!("{}", line);

    println!("INJECTION TECHNIQUES:");
    let techniques = vec![
        (
            "[1] Thread Hijacking",
            "  Suspends a host process, injects shellcode\n\t     into remote memory, and redirects the\n\t     thread's execution flow via RIP/EIP\n\t     register modification."
        ),
        (
            "[2] Remote Thread Execution",
            "  Allocates memory in a target process and\n\t     uses CreateRemoteThread to\n\t     execute shellcode."
        ),
        (
            "[3] Module Overloading (Coming soon...)",
            "  Loads a legitimate DLL into the target\n\t     process and overwrites its memory space\n\t     with the malicious payload."
        ),
    ];

    for (title, desc) in techniques {
        println!("\n\t{} -", title);
        println!("\t   {}", desc);
    }

    println!("\n{}", sub_line);

    println!("INTERNAL PAYLOADS:");
    let payloads = vec![
        (
            "messagebox",
            "A standard WinAPI payload that triggers\n\t     a graphical message box on the screen\n\t     once the injection is successful."
        ),
        (
            "calculator",
            "A proof-of-concept payload that spawns\n\t     the Windows Calculator (calc.exe) process\n\t     from the target host."
        ),
    ];

    for (name, p_desc) in payloads {
        println!("\n\t* {} -", name);
        println!("\t   {}", p_desc);
    }

    println!("\n{}", sub_line);

    println!("BYPASS LEVEL DEFINITIONS:");
    let bypass_levels = vec![
        (
            "Basic Level",
            "90% Detection Rate",
            vec!["Standard WinAPI calls", "Plaintext strings"]
        ),
        (
            "Moderate Level",
            "50% Bypass Rate",
            vec!["XOR String Obfuscation", "Payload Encryption", "API Dynamic Resolution"]
        ),
        (
            "Difficult Level",
            "70% Bypass Rate",
            vec!["ETW Patching", "AMSI Patching", "NTAPI / Direct Syscalls"]
        ),
    ];

    for (level, rate, features) in bypass_levels {
        println!("\n\t{} ({}):", level, rate);
        for feature in features {
            println!("\t   [+] {}", feature);
        }
    }
    
    println!("\n{}\n", line);
}