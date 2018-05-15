extern crate clap;
extern crate colored;
//extern crate regex;
//extern crate serde_json;
//#[macro_use]
//extern crate chan;
//extern crate chan_signal;
#[macro_use]
extern crate lazy_static;

use std::fs::OpenOptions;
use std::io::Read;
use std::process::Command;
use colored::*;
//use serde_json::Value;
//use std::thread;
//use chan_signal::Signal;
use std::sync::RwLock;
use clap::{App, Arg, SubCommand};

struct SystemResult {
    stdout: String,
    stderr: String,
    status: i32
}

impl SystemResult {
    fn new(output: std::process::Output) -> SystemResult {
        let mut stdout: Vec<char> = std::str::from_utf8(&output.stdout[..]).unwrap().to_string().chars().collect();
        stdout.pop();
        let stdout: String = stdout.into_iter().collect();
        let mut stderr: Vec<char> = std::str::from_utf8(&output.stderr[..]).unwrap().to_string().chars().collect();
        stderr.pop();
        let stderr: String = stderr.into_iter().collect();
        let mut result = SystemResult {
            stdout: stdout,
            stderr: stderr,
            status: 0
        };
        if result.stderr.chars().count() > 0 {
            result.status = 1
        }
        result
    }
}

fn system(command: &str) -> SystemResult {
    let result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    let result = SystemResult::new(result);
    if result.status != 0 {
        let emsg = [
            "== ".red().to_string(),
            "[+]ERROR".red().bold().to_string(),
            " =====================".red().to_string()
        ].join("");
        println!("{}", emsg);
        println!("{}", result.stderr);
        println!("{}", "=================================".red().to_string());
    }
    result
}

fn get_stdout(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    let mut stdout: Vec<char> = std::str::from_utf8(&output.stdout[..]).unwrap().to_string().chars().collect();
    stdout.pop();
    let stdout: String = stdout.into_iter().collect();
    stdout
}

fn system_allow_stderr(command: &str) -> SystemResult {
    let result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");
    SystemResult::new(result)
}

fn process(command: &str) -> std::process::ExitStatus {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("failed to execute process");
    child.wait().unwrap()
}

lazy_static! {
    static ref ADDRESS: RwLock<String> = RwLock::new(String::new());
    static ref SUM: RwLock<f64> = RwLock::new(0.00);
    static ref LOOP_COUNTER: RwLock<f64> = RwLock::new(1.00);
}

fn help() {
    println!("\
USAGE:
    manukazeny [SUBCOMMAND]
manukazeny -h for help\
");
}

fn pmanage(process_num: &str, start_command: &str, kill_command: &str) {
    let process_num: i32 = process_num.parse().unwrap();
    let vmstat_out = get_stdout("vmstat");
    let vmstat_vec: Vec<&str> = vmstat_out.split("\n").collect();
    let vmstat_out = vmstat_vec[2];
    let vmstat_chars: Vec<char> = vmstat_out.chars().collect();
    let r = vmstat_chars[1];
    let r: i32 = r as i32;
    let mut data = String::new();
    let file_name = "/tmp/pmanager.tmp";
    let mut f = match OpenOptions::new().read(true).open(file_name) {
        Ok(f) => f,
        Err(_) => {
            print!("Can not open file: ");
            println!("{}", file_name);
            process("echo killed > /tmp/pmanager.tmp");
            println!("Created.");
            let mut f = match OpenOptions::new().read(true).open(file_name) {
                Ok(f) => f,
                Err(_) => {
                    print!("Can not open file: ");
                    println!("{}", file_name);
                    std::process::exit(0);
                }
            };
            f
        }
    };
    f.read_to_string(&mut data).expect(["Can not read file: ", file_name].join("").as_str());
    if (r >= 0x30 + process_num) {
        if (data == "started") {
            process(["echo -n killed > ", file_name].join("").as_str());
            process(kill_command);
        }
    } else {
        if (data == "killed") {
            process(["echo -n started > ", file_name].join("").as_str());
            process(start_command);
        }
    }
}

fn main() {
    let matches = App::new("pmanager")
        .version("1.0.0")
        .author("miyagaw61 <miyagaw61@gmail.com>")
        .about("process manager")
        .arg(Arg::with_name("process-num")
             .takes_value(true)
             )
        .arg(Arg::with_name("start-command")
             .takes_value(true)
             )
        .arg(Arg::with_name("kill-command")
             .takes_value(true)
             )
        .get_matches();
    pmanage(matches.value_of("process-num").unwrap(),
        matches.value_of("start-command").unwrap(),
        matches.value_of("kill-command").unwrap());
}
