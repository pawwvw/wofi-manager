use std::process::{Command, Stdio};
use std::io::Write;
use anyhow::Result;
use clap::Parser;
use std::fs;
use serde_json::Value;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    options: String
}

fn main() -> Result<()> {

    let args = Args::parse();

    let options_path = args.options;

    let options_file = fs::read_to_string(options_path)?;

    let options_json: Value = serde_json::from_str(&options_file)?;

    let options = options_json.as_object().unwrap();
    let mut wofi = Command::new("wofi")
        .arg("--dmenu")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("sdls");

    let mut input = String::new();
    for (key, _) in options.iter().rev() {
        input = input + &format!("{}\n", key);
    }
    wofi.stdin.as_mut().unwrap().write_all(input.trim().as_bytes()).expect("slkd");
    let result = wofi.wait_with_output().expect("sldls");
    let com = options.get(String::from_utf8(result.stdout)?.trim()).unwrap().as_str().unwrap_or("").to_string();
    let com_arr: Vec<&str> = com.split(" ").collect();
    let program = &com_arr.get(0).unwrap();
    let args = &com_arr[1..];
    
    Command::new(&program).args(args).spawn()?;

    Ok(())
}
