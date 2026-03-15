# Chimera Framework
![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)
![Language](https://img.shields.io/badge/Language-Rust-orange.svg)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20Termux-lightgrey.svg)

**Chimera** is a high-performance Proof of Concept (PoC) framework developed in Rust, engineered for validating **Shellcode Injection** techniques. Its primary purpose is to provide security researchers and malware analysts with a controlled environment to study process internals, memory manipulation, and execution vectors such as Thread Hijacking and Remote Thread Injection across dual-architecture (x86/x64) systems.

---

## 🛠 Prerequisites & Installation

### 🟦 Windows
1. **Install Git:** Download and install from [git-scm.com](https://git-scm.com/downloads).
2. **Install Rust:** Download `rustup-init.exe` from [rustup.rs](https://rustup.rs/) and follow the installation prompts.
3. **Install GNU Toolchains:**
   Open your terminal (PowerShell or CMD) and run:
   ```powershell
   rustup target add x86_64-pc-windows-gnu
   rustup target add i686-pc-windows-gnu

```

4. **Clone & Run:**
```powershell
git clone https://github.com/sec-johnphilipjunio/chimera.git
cd chimera
cargo run

```



---

### 🟩 Linux (Ubuntu/Debian Based)

1. **Update System & Install Dependencies:**
```bash
sudo apt update && sudo apt upgrade -y
sudo apt install build-essential git gcc-mingw-w64 -y

```


2. **Install Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
source $HOME/.cargo/env

```


3. **Add Windows Targets:**
```bash
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu

```


4. **Clone & Run:**
```bash
git clone https://github.com/sec-johnphilipjunio/chimera.git
cd chimera
cargo run

```



---

### 🟧 Termux (Android)

1. **Install Base Packages:**
```bash
pkg update && pkg upgrade
pkg install rust git binutils -y

```


2. **Add Windows Targets:**
```bash
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu

```


3. **Clone & Run:**
```bash
git clone https://github.com/sec-johnphilipjunio/chimera.git
cd chimera
cargo run

```



---

## 🚀 Purpose & Focus

The Chimera Framework is built specifically for **Proof of Concept (PoC)** scenarios. Its core functionality focuses on:

* **Shellcode Delivery:** Automating the synchronization of `.bin` payloads into specialized injection templates.
* **Technique Validation:** Demonstrating the mechanics of Windows API interactions through methods like Remote Thread Injection and Thread Context Hijacking.
* **Research & Education:** Providing a modular codebase to help researchers understand memory-resident execution, stealth patterns, and cross-architecture compatibility.

---

## ⚖ License

This project is licensed under the **Apache License 2.0**.

> **Disclaimer:** This tool is for educational and authorized security testing purposes only. Using this tool against targets without prior written consent is illegal. The developers assume no liability and are not responsible for any misuse or damage caused by this program.