use std::fs;
use std::path::{Path, PathBuf};
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct JsonSynthEntry {
    synth_type: String,
    frequency: f32,
    name: String
}

struct SynthAction {
    frequency: f32,
    name: String,
    start_time: usize,
    end_time: usize

}

pub struct ScriptParser {
    path: String,
    files: Vec<String>,
    actions: Vec<SynthAction>
}

impl ScriptParser {
    pub fn new(path: String) -> ScriptParser {
        let mut files: Vec<String> = Vec::new();
        let mut actions: Vec<SynthAction> = Vec::new();
        let paths = fs::read_dir(&path).unwrap();
        for p in paths {
            let s = p.unwrap().path().to_str().unwrap().to_string();
            if PathBuf::from(&s).is_file() {
                files.push(s);
            }

        }
        ScriptParser {
            path,
            files,
            actions
        }
    }

    pub fn print_files(&self) {
        for f in &self.files {
            println!("{f}");
        }
    }

    pub fn parse_files(&self) {
        self.parse_setup_json();
        self.parse_script_files();
    }

    pub fn parse_setup_json(&self) {
        let setup_path_string = format!("{}/setup.json", self.path);
        let setup_path = Path::new(setup_path_string.as_str());
        if setup_path.exists() {
            let ds: Value = serde_json::from_str(fs::read_to_string(setup_path).unwrap().as_str()).expect("Error while reading setup.json");
            println!("setup.json loaded!");
            for synth in ds["synths"].as_array().unwrap() {
                let s: JsonSynthEntry = serde_json::from_str(synth.to_string().as_str()).unwrap();
                println!("{}", s.synth_type);
                println!("{}", s.frequency);
            }
        }
        else {
            println!("setup.json not found!");
        }
    }

    pub fn parse_script_files(&self) {
        for f in &self.files {
            let file = Path::new(f);
            //let extension = file.extension().unwrap();
            //let t = fs::read_to_string(file).unwrap();

            if let Some(extension) = file.extension() {
                if extension.to_str().unwrap() == "gs" {
                    println!("GScript file found!");
                    let content = fs::read_to_string(file);
                    for line in content.unwrap().split("\n") {
                        println!("{line}");
                    }
                }
            }
        }

    }
    fn parse_gscript_line(&self, line: String) {
        let splitted: Vec<&str> = line.split(";").collect();
        let name = splitted[0].to_string();
        let freq = splitted[1].to_string();
        let start = splitted[2].to_string();
        let end = splitted[3].to_string();
    }
}