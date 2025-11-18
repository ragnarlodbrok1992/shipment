extern crate regex;

use std::process::Command;
use std::fs;
use std::env;
use std::fs::File;
use std::io::Write;
use regex::Regex;

fn blob_to_lines(blob: &String) -> Vec<&str> {
    let re = Regex::new(r"\r\n").unwrap();
    let lines_of_blob: Vec<&str> = re.split(&blob).collect();
    let trimmed_lines = lines_of_blob.iter().map(|s| s.trim_matches('"')).collect();
    // TODO(moliwa): fix this - lines do not get trimmed for ""?
    trimmed_lines
}

fn main() {
    eprintln!("DEBUG --> this is how to output debug strings in build.rs script");
    eprintln!("DEBUG --> to see those messages You need to run with double verbosity: cargo build -vv");

    let only_windows_warning = "This build script works on Windows only!";

    let mut log_file = File::create("build_script_output.txt").unwrap();
    let vcvars_bat_file = "vcvars.bat";
    let compiler = "cl.exe";
    let lib_bin = "lib.exe";
    let output_dir = "win_build";
    let windows_code_test_binary = "shipment_win32_test.exe";
    let windows_layer_lib = "windows_layer_library";
    fs::create_dir_all(output_dir).unwrap();

    // Source files for c++ build
    let source_file = "src/shipment.cpp";
    

    // Step 0. Running vcvar.bat file
    writeln!(log_file, "\n\nRunning vcvars.bat file and setting up environment...\n\n").unwrap();
    let output_stdout = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");

        cmd.arg("/C")
           .arg(vcvars_bat_file)
           .arg("&&")
           .arg("set"); // Printing environment variables

        let output = cmd.output().expect("Failed to execute vcvars.bat file!");
        if !output.status.success() {
            panic!("Failed to setup MSVC environment: {:?}", output);
        }

        let env_vars = String::from_utf8_lossy(&output.stdout);
        for line in env_vars.lines() {
            if let Some((key, value)) = line.split_once('=') { unsafe
                {
                    env::set_var(key, value);
                }
            }
        }
        &output.stdout.clone()
    } else {
        panic!("{}", only_windows_warning);
    };

    for line in blob_to_lines(&String::from_utf8(output_stdout.to_vec()).unwrap()) {
        writeln!(log_file, "{:?}", line).unwrap();
    }

    // Step 1. Building test binary for C++ windows code
    // After enabling environment try to run cl.exe
    writeln!(log_file, "\n\nRunning cl.exe to build test binary...\n").unwrap();
    let compiler_output = if cfg!(target_os = "windows") {
        let fe_string = format!("/Fe{}/{}", output_dir, windows_code_test_binary);
        let fo_string = format!("/Fo{}/", output_dir);
        let fd_string = format!("/Fd{}/", output_dir);

        let mut cmd = Command::new(compiler);
        cmd.arg("/EHsc")
           .arg("/Zi")
           .arg(fe_string)
           .arg(fo_string)
           .arg(fd_string)
           .arg(source_file)
           .arg("/link")
           .arg("user32.lib")
           .arg("gdi32.lib");

        let output = cmd.output().expect("Failed to execute command cl.exe! --> building test binary!");

        if !output.status.success() {
            // DEBUG
            eprintln!("Current dir: {}", std::env::current_dir().unwrap().display());

            panic!("\n\n\nFailed to run compiler! {:?}\n\n\n", output);
        }

        &output.stdout.clone()
    } else {
        panic!("{}", only_windows_warning)
    };

    for line in blob_to_lines(&String::from_utf8(compiler_output.to_vec()).unwrap()) {
        writeln!(log_file, "{:?}", line).unwrap();
    }

    // Step 2a. Building library - windows_layer
    writeln!(log_file, "\n\nRunning cl.exe to build library...\n").unwrap();
    let windows_layer_output = if cfg!(target_os = "windows") {
        let mut cmd = Command::new(compiler);
        let windows_layer_lib_source = "src/windows_layer.cpp";
        let fo_string = format!("/Fo{}/{}.obj", output_dir, windows_layer_lib);

        cmd.arg("/c")
           .arg(fo_string)
           .arg(windows_layer_lib_source);

        let output = cmd.output().expect("Failed to execute command cl.exe --> building windows layer obj file!");

        if !output.status.success() {
            panic!("\n\n\nFailed to run compiler for windows layer library! {:?}\n\n\n", output);
        }

        &output.stdout.clone()
    } else {
        panic!("{}", only_windows_warning)
    };

    for line in blob_to_lines(&String::from_utf8(windows_layer_output.to_vec()).unwrap()) {
        writeln!(log_file, "{:?}", line).unwrap();
    }

    // Step 3. Linking obj file to .lib file
    writeln!(log_file, "\n\nRunning lib.exe to link to .lib static library file...\n\n").unwrap();
    let link_output = if cfg!(target_os = "windows") {
        let mut cmd = Command::new(lib_bin);
        let obj_file = format!("{}/{}.obj", output_dir, windows_layer_lib);
        let output_lib_file = format!("/out:{}/{}.lib", output_dir, windows_layer_lib);

        cmd.arg(obj_file)
           .arg(output_lib_file)
           .arg("/NOENTRY");

        let output = cmd.output().expect("Failed to execute lib.exe --> building .lib file!");

        if !output.status.success() {
            panic!("\n\n\nFailed to run lib.exe for lib files! {:?}\n\n\n", output);
        }

        &output.stdout.clone()
    } else {
        panic!("\n\n\nFailed to execute command link.exe! --> creating static libary!");
    };

    for line in blob_to_lines(&String::from_utf8(link_output.to_vec()).unwrap()) {
        writeln!(log_file, "{:?}", line).unwrap();
    }

    // Finally - link build .lib with our Rust code
    println!("{}", format!("cargo:rustc-link-search={}/", output_dir));
    println!("{}", format!("cargo:rustc-link-lib=static={}", windows_layer_lib));
}
