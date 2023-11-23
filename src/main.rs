use std::fs::File;
use std::env::current_dir;

fn main() {
    let cwd = current_dir().unwrap().join("src-tauri").join("tauri.conf.json");
    let file = File::open(cwd).unwrap();
    let data: serde_json::Value = serde_json::from_reader(file).unwrap();
    let version = data["package"]["version"].to_string().replace("\"", "");
    println!("{}", version);
}