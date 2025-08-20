use std::process::{Command, Stdio};
use std::io::Write;
use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut hm = HashMap::new();
    hm.insert("firefox", "e");
    hm.insert("dolphin", "dolphin");
    let mut wofi = Command::new("wofi")
        .arg("--dmenu")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("sdls");

    let mut input = String::new();
    for (key, _) in hm.iter() {
        input = input + &format!("{}\n", key);
    }
    wofi.stdin.as_mut().unwrap().write_all(input.trim().as_bytes()).expect("slkd");
    let result = wofi.wait_with_output().expect("sldls");
    let com = hm.get(String::from_utf8(result.stdout)?.trim()).unwrap();
    let com_arr: Vec<&str> = com.split(" ").collect();
    let program = &com_arr.get(0).unwrap();
    let args = &com_arr[1..];
    
    Command::new(&program).args(args).spawn()?;

    Ok(())
}
