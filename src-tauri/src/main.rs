// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod hello_rust;
mod variables;

fn main() {
    captain_lib::run()
}
