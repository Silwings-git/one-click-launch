// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use one_click_start_lib::resource::ResourceLocation;

fn main() {
    let s = vec![ResourceLocation::FilePath(PathBuf::from("main.json"))];
    
    let s = serde_json::to_string(&s).unwrap();
    
    println!("{}", s);

    // one_click_start_lib::run()
}
