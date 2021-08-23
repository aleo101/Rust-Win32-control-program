# Rust-Win32-control-program
Windows API (Win32) program written in Rust to get practice with Rust and Windows programming.

I believe this project can help you get started on making your own Windows API application in Rust!

## Getting Started:
This should run on any windows computer that has Rust installed. 
Run the program by navigating to the directory in PowerShell and run the commands: 
```cmd
cargo build
cargo run
```
If all goes to plan, the executable (.exe) will be compiled and the GUI should pop up as a window.

## What is it?
This Windows GUI application is written in Rust using the new officially supported ``windows`` crate [(link to crate)](https://github.com/microsoft/windows-rs).

## How does it work?
The program works by making calls directly to Windows API user interface functions. 

## Currently includes:
  - Buttons.
  - Edit control fields (editable text-boxes).
  - A scroll-bar (slider control bar).
 
