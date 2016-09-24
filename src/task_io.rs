extern crate serde;
extern crate serde_json;
use task::Task;
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf};
use std::env;

pub fn read_tasks() -> Result<Vec<Task>, String> {
    let path = try!(get_task_store());
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut file = try!(
        File::open(&path).map_err(|e| e.to_string())
    );
    let mut json = String::new(); try!(
        file.read_to_string(&mut json).map_err(|e| e.to_string())
    );
    let tasks: Vec<Task> = try!(
        serde_json::from_str(&json).map_err(|e| e.to_string())
    );
    return Ok(tasks);
}

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), String>  {
    let json = try!(
        serde_json::to_string(&tasks).map_err(|e| e.to_string())
    );
    let path = try!(get_task_store());
    let mut file = try!(
        File::create(&path).map_err(|e| e.to_string())
    );
    try!(
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())
    );
    return Ok(());
}

fn get_task_store() -> Result<PathBuf, String> {
    let home_dir = try!(env::home_dir().ok_or("Could not determine home directory"));
    let path = home_dir.join(".tasks.json");
    return Ok(path);
}
