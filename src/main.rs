use std::env::Args;
use std::env;
use clap::{App, Arg};
use std::path::{PathBuf, Path};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write, BufWriter};
use ruscripter::{Config, Script, get_config_path};
use serde_yaml::Error;
use cursive::Cursive;
use cursive::views::{SelectView, BoxView, Dialog, TextArea, HideableView, IdView};
use cursive::align::HAlign;
use cursive::view::Boxable;
use cursive::view::Identifiable;
use cursive::event::Event::Key;
use std::collections::hash_map::Keys;
use cursive::event::Key::Tab;
use cursive::view::ViewWrapper;
use std::process::{Command, Stdio};
use std::str;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("ruscripter")
        .version("0.1").author("Tyler Wickline")
        .arg(Arg::with_name("CONFIG")
            .help("Specify location of the config file. (ruscript_config.yaml)")
            .takes_value(true)
            .default_value("./")
            .index(1))
        .arg(Arg::with_name("init")
            .short("i")
            .help("Create an example project from the given path")
            .takes_value(true)
        )
        .get_matches();
    if matches.is_present("init") {
        let path = matches.value_of("init").unwrap();
        Config::init(PathBuf::from(path));
        return Ok(());
    }

    let config_file = File::open(get_config_path().as_path())?;
    let mut config = Config::new(config_file);
    let mut siv = Cursive::ncurses().unwrap();
    let mut select: SelectView<Script> = SelectView::new().autojump();
    for (str, script) in config.build_list() {
        select.add_item(str, script);
    }
    select.set_on_submit(handle_select);
    siv.add_fullscreen_layer(select.full_screen());


    siv.run();
    Ok(())
}

fn handle_select(siv: &mut Cursive, item: &Script) {
    let output = Command::new(item.path.as_str())
        .output();
    if output.is_err() {
        clearAndWriteBytes(output.unwrap_err().to_string().as_bytes())
    } else {
        clearAndWriteBytes(output.unwrap().stdout.as_ref())
    }
}

fn clearAndWriteBytes(mut buf: &[u8]) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("text.txt")
        .unwrap();
    file.set_len(0);
    file.write_all(buf).unwrap();
}


