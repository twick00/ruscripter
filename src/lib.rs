use serde::{Deserialize, Serialize};
use std::path::{PathBuf, Iter};
use clap::{App, Arg, ArgMatches};
use std::fmt::{Display, Formatter, Error};
use std::fs::File;
use std::io::{BufReader, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub project_name: Option<String>,
    pub description: Option<String>,
    pub scripts: Vec<Script>,
    pub commands: Option<Vec<Command>>,
    #[serde(skip)]
    long_name: usize,
    #[serde(skip)]
    long_path: usize,
    #[serde(skip)]
    long_desc: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    pub name: String,
    pub description: String,
    pub path: String,
}

impl Config {
    pub fn init(path:PathBuf) {
        let config = Config {
            project_name: Some("Example Project Name".to_string()),
            description: Some("Example Project Description".to_string()),
            scripts: vec![
                Script{
                    name: "test script 1".to_string(),
                    description: "test description 1".to_string(),
                    path: "./test/path1".to_string()
                },
                Script{
                    name: "test script 2".to_string(),
                    description: "test description 2".to_string(),
                    path: "./test/path2".to_string()
                }
            ],
            commands: None,
            long_name: 0,
            long_path: 0,
            long_desc: 0
        };
        let output = serde_yaml::to_string(&config).unwrap();
        let mut file = match File::open(&path) {
            Ok(file) =>{
                file
            }
            Err(_) => {
                File::create(&path).unwrap()
            }
        };
        file.write_all(output.as_ref());

    }
    pub fn build_list(&self) -> Vec<(String, Script)> {
        let mut out = vec![];
        for (idx, script) in self.scripts.iter().enumerate() {
            let text = format!("[{: <3}] {: <long_name$}   :   {: <long_path$}   :   {: <long_desc$}", idx + 1, script.name.as_str(), script.path.as_str(), script.description.as_str(),
                               long_name = self.long_name, long_path = self.long_path, long_desc = self.long_desc);

            //TODO: Fix this, it unnecessarily duplicates the script objects
            out.push((text.to_string(), Script{
                name: script.name.to_string(),
                description: script.description.to_string(),
                path: script.path.to_string()
            }))
        }
        out
    }
    fn set_lengths(&mut self) {
        //Set lengths of longest of each for rendering the text nicely
        let mut long_name: usize = 0;
        let mut long_path: usize = 0;
        let mut long_desc: usize = 0;
        for script in &self.scripts {
            if script.description.len() > long_desc {
                long_desc = script.description.len();
            }
            if script.path.len() > long_path {
                long_path = script.path.len();
            }
            if script.name.len() > long_name {
                long_name = script.name.len();
            }
        }
        self.long_name = long_name;
        self.long_path = long_path;
        self.long_desc = long_desc;
    }

    pub fn new(file: File) -> Self {
        let mut config: Config = serde_yaml::from_reader(BufReader::new(file)).unwrap();
        config.set_lengths();
        config
    }
}

pub fn get_config_path() -> PathBuf {
    let path_value = "./";
    let matches = App::new("ruscripter")
        .version("0.1").author("Tyler Wickline")
        .arg(Arg::with_name("CONFIG")
            .help("Specify location of the config file. (ruscript_config.yaml)")
            .takes_value(true)
            .default_value(path_value)
            .index(1))
        .arg(Arg::with_name("init")
            .short("i")
            .help("Create an example project from the given path")
            .takes_value(true)
            .default_value("./")
        )
        .get_matches();
    let mut path_value = PathBuf::from(matches.value_of("CONFIG").unwrap());
    if path_value.is_dir() {
        path_value.push("ruscript_config.yaml");
    }
    if !path_value.exists() {
        panic!("Could not find ruscript_config.yaml file located at {}", path_value.display())
    }
    path_value
}