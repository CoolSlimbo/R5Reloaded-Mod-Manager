#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![getMapNames, obtainModsInFolder, launchGame])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

use std::env;
use std::fs::{OpenOptions, read_dir};
use std::io::Read;
use std::path::Path;
use glob::{glob, Pattern};
use regex::Regex;
use std::process::Command;


#[tauri::command]
fn getMapNames() -> Vec<String> {
  let mut map_names = Vec::new();

  let exe_path = env::current_dir().unwrap().to_string_lossy().to_string();
  let map_folder = exe_path.clone() + "//vpk//*.bsp.pak000_dir.vpk";

  for entry in glob(&map_folder.to_string()).unwrap() {
    match entry {
      Ok(path) => map_names.push(regex_stbsp(path.display().to_string())),
      Err(e) => map_names.push(e.to_string()),
    }
  }

  return map_names;
}

fn regex_stbsp(str: String) -> String {
  let vpk_remover = Regex::new(r"(\.bsp.pak000_dir.vpk$)").unwrap();
  let slash_remover = Regex::new(r"(.*\\)").unwrap();
  let englishclient_remover = Regex::new(r"(englishclient_)").unwrap();

  let text_vpk = vpk_remover.replace_all(&str, "");
  let text_slash = slash_remover.replace_all(&text_vpk, "").to_string();
  let text_englishclient = englishclient_remover.replace_all(&text_slash, "").to_string();

  return text_englishclient.to_string();
  //return str.to_string();
}

#[tauri::command]
fn obtainModsInFolder() -> Vec<String> {
  let mut modsName = Vec::new();

  let exe_path = env::current_dir().unwrap().to_string_lossy().to_string();
  let mods_folder = exe_path.clone() + "\\mods\\";

  for entry in read_dir(&mods_folder.to_string()).unwrap() {
    match entry {
      Ok(path) => modsName.push(removeSlash(path.path().display().to_string())),
      Err(e) => modsName.push(e.to_string()),
    }
  }

  //println!("{:?}", modsName);

  // if (modsName.index_of("enabled.json").unwrap() != -1) {
  //   modsName.remove(modsName.index_of("enabled.json").unwrap());
  // }

  // For the length of modsName, open mod.json, located in mods_folder + modsName and push it to a vector

  let mut mod_json = Vec::new();
  
  for i in &modsName {
    //println!("{}", modName);
    let mod_json_path = mods_folder.to_string() + &i + "\\mod.json";

    if !Path::new(&mod_json_path).exists() {
      mod_json.push(String::from("Err. 2") + &i);
    } else {
      /// Open the file in read-only mode (ignoring errors).
      let mut file = OpenOptions::new()
        .read(true)
        .open(&mod_json_path)
        .unwrap();

      let mut contents = String::new();
      file.read_to_string(&mut contents).unwrap();

      mod_json.push(contents);
    }
  }

  return mod_json;
}

fn removeSlash(str: String) -> String {
  let slash_remover = Regex::new(r"(.*\\)").unwrap();
  let text_slash = slash_remover.replace_all(&str, "").to_string();

  return text_slash;
}

#[tauri::command]
fn launchGame(arg: String) {
  let exe_path = env::current_dir().unwrap().to_string_lossy().to_string();
  Command::new(exe_path.clone() + "\\launcher.exe")
    .current_dir(exe_path.clone())
    .arg(arg)
    .spawn()
    .expect("Failed to start launcher.exe");
}