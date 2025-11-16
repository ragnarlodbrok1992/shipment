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
    trimmed_lines
}

fn main() {
    eprintln!("DEBUG --> this is how to output debug strings in build.rs script");
    eprintln!("DEBUG --> to see those messages You need to run with double verbosity: cargo build -vv");

    let mut log_file = File::create("build_script_output.txt").unwrap();
    let vcvars_bat_file = "vcvars.bat";
    let compiler = "cl.exe";
    let output_dir = "win_build";
    fs::create_dir_all(output_dir).unwrap();

    // Source files for c++ build
    let source_file = "src/shipment.cpp";
    

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
        panic!("This build script works on Windows only!");
    };

    for line in blob_to_lines(&String::from_utf8(output_stdout.to_vec()).unwrap()) {
        writeln!(log_file, "{:?}", line).unwrap();
    }

    writeln!(log_file, "\n\nRunning cl.exe...\n").unwrap();
    // After enabling environment try to run cl.exe
    let compiler_output = if cfg!(target_os = "windows") {
        let fe_string = format!("/Fe{}/hello_world.exe", output_dir);
        let fo_string = format!("/Fo{}/", output_dir);
        let fd_string = format!("/Fd{}/", output_dir);

        let mut cmd = Command::new(compiler);
        cmd.arg("/EHsc")
           .arg("/Zi")
           // .arg(r#"/Fewin_build/hello_world.exe"#)
           // .arg(r#"/Fowin_build/"#)
           // .arg(r#"/Fdwin_build/"#)
           .arg(fe_string)
           .arg(fo_string)
           .arg(fd_string)
           .arg(source_file)
           .arg("/link")
           .arg("user32.lib");

        let output = cmd.output().expect("Failed to execute command cl.exe!");

        if !output.status.success() {
            // DEBUG
            eprintln!("Current dir: {}", std::env::current_dir().unwrap().display());

            panic!("\n\n\nFailed to run compiler! {:?}\n\n\n", output);
        }

        &output.stdout.clone()
    } else {
        panic!("This build script works on Windows only!")
    };

    for line in blob_to_lines(&String::from_utf8(compiler_output.to_vec()).unwrap()) {
        writeln!(log_file, "{:?}", line).unwrap();
    }
}
