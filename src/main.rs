use anyhow::{Result, Context};
use clap::Parser;
use serde_json::{Value, Map};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    options: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let options_path = args.options;

    let options_file = fs::read_to_string(options_path)
        .context("Don`t find your json config for menu")?;
    let options_json: Value = serde_json::from_str(&options_file)?;

    let options = options_json.as_object().unwrap();
    let mut wofi = Command::new("wofsd")
        .arg("--dmenu")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("You must have wofi")?;

    let options_string = get_options_string(&options);
    wofi.stdin
        .as_mut()
        .unwrap()
        .write_all(options_string.trim().as_bytes())
        .context("")?;
    let result = wofi.wait_with_output().context("")?;
    let com = options
        .get(String::from_utf8(result.stdout)?.trim())
        .unwrap()
        .as_str()
        .unwrap_or("")
        .to_string();
    let com_arr: Vec<&str> = com.split(" ").collect();
    let program = &com_arr.get(0).unwrap();
    let args = &com_arr[1..];

    Command::new(&program).args(args).spawn()?;

    Ok(())
}


fn get_options_string(options: &&Map<String, Value>)->String {
    let mut input = String::new();
    for (key, _) in options.iter() {
        input = input + &format!("{}\n", key);
    }
    input
}
