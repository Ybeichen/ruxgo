use std::{fs::File, io::Read};
use toml::Table;
use colored::Colorize;

//Log utils
#[derive(PartialEq, PartialOrd, Debug)]
pub enum LogLevel {
    Info,
    Log,
    Warn,
    Error,
}

pub fn log(level: LogLevel, message: &str) {
    let level_str = match level {
        LogLevel::Info => "[INFO]".blue(),
        LogLevel::Log => "[LOG]".green(),
        LogLevel::Warn => "[WARN]".yellow(),
        LogLevel::Error => "[ERROR]".red(),
    };
    let log_level = match std::env::var("RUKOSKIT_LOG_LEVEL") {
        Ok(val) => {
            if val == "Info" {
                LogLevel::Info
            } else if val == "Log" {
                LogLevel::Log
            } else if val == "Warn" {
                LogLevel::Warn
            } else if val == "Error" {
                LogLevel::Error
            } else {
                LogLevel::Log
            }
        }
        Err(_) => LogLevel::Log,
    };
    if level >= log_level {
        println!("{} {}", level_str, message);
    }
}

//Toml utils
#[derive(Debug)]
pub struct BuildConfig {
    pub compiler: String,
    pub build_dir: String,
    pub obj_dir: String,
}

#[derive(Debug)]
pub struct TargetConfig {
    pub name: String,
    pub src: String,
    pub include_dir: String,
    pub typ: String,
    pub cflags: String,
    pub libs: String,
}

pub fn parse_config(path: &str) -> (BuildConfig, Vec<TargetConfig>) {
    // open toml file and parse it into a string
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config = contents.parse::<Table>().unwrap();

    // parse the string into a struct
    let build_config = BuildConfig {
        compiler: config["build"]["compiler"].as_str().unwrap().to_string(),
        build_dir: config["build"]["build_dir"].as_str().unwrap().to_string(),
        obj_dir: config["build"]["obj_dir"].as_str().unwrap().to_string(),
    };

    // considering multiple targets
    let mut targets = Vec::new();   

    for target in config["targets"].as_array().unwrap() {
        let target_config = TargetConfig {
            name: target["name"].as_str().unwrap().to_string(),
            src: target["src"].as_str().unwrap().to_string(),
            include_dir: target["include_dir"].as_str().unwrap().to_string(),
            typ: target["type"].as_str().unwrap().to_string(),
            cflags: target["cflags"].as_str().unwrap().to_string(),
            libs: target["libs"].as_str().unwrap().to_string(),
        };
        targets.push(target_config);
    }

    (build_config, targets)
}