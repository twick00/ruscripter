use std::env::Args;
use std::env;
use clap::{App, Arg};
use std::path::{PathBuf, Path};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufRead, Write, BufWriter, Read};
use ruscripter::{Config, Script, get_config_path};
use serde_yaml::Error;
use cursive::Cursive;
use cursive::views::{SelectView, BoxView, Dialog, TextArea, HideableView, IdView, TextView, LinearLayout, DebugView, SliderView};
use cursive::align::HAlign;
use cursive::view::Boxable;
use cursive::view::Identifiable;
use cursive::event::Event::Key;
use std::collections::hash_map::Keys;
use cursive::event::Key::Tab;
use cursive::view::ViewWrapper;
use std::process::{Command, Stdio, exit, Output, Child};
use std::str;
use cursive::direction::Orientation;
use cursive::direction::Orientation::Vertical;
use cursive::view::Scrollable;
use std::hint::unreachable_unchecked;

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
    cursive::logger::init();

    let mut select: SelectView<Script> = SelectView::new().autojump();
    let list = config.build_list();
    let list_size = list.len();
    let mut longest_size = 0;
    for (str, script) in list {
        if str.len() > longest_size {
            longest_size = str.len();
        }
        select.add_item(str, script);
    }
    select.set_on_submit(handle_select);
    let select = select.scrollable().scroll_x(true).max_height(10);
    let mut layout = LinearLayout::new(Vertical);
    layout.add_child(select);
    layout.add_child(
        Dialog::around(
            TextView::new("Run a script to see output")
                .h_align(HAlign::Left)
                .with_id("output_box")
                .full_height()).title("Script Output").title_position(HAlign::Right));
    layout.add_child(cursive::views::Dialog::text("Press ~ to open the console.").fixed_height(3));
    layout.add_child(DebugView::new().with_id("debug").fixed_height(20));

    siv.add_global_callback('s', |s| {
        s.toggle_debug_console()
    });
    siv.add_global_callback(Key(Tab), Cursive::quit);
    siv.add_fullscreen_layer(layout.full_screen());

    siv.run();

    Ok(())
}

fn handle_select(siv: &mut Cursive, item: &Script) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(item.path.as_str())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn().unwrap();
    let mut stream_buffer:&mut String = &mut "".to_string();
    let mut output_box = siv.find_id::<TextView>("output_box").unwrap();
    output_box.set_content("");
    for line in  io::stdin().lock().lines() {
        if line.is_ok() {
            output_box.append(line.unwrap());
        }
    }
//    String::from_utf8(stream_buffer.to_vec())
//        output
//    {
//            Ok(out) => {}
//        Child {handle: handle, status: status, stdout: stdout, stderr: stderr } => {
////            clear_and_write_bytes(stdout.as_ref());
//            let mut output_box = siv.find_id::<TextView>("output_box").unwrap();
//            if stderr.is_empty() {
//                BufReader::new(String::from_utf8(stdout.unwrap()).unwrap());
////                output_box.set_content(String::from_utf8(stdout).unwrap());
//            } else {
//                output_box.set_content(String::from_utf8(stderr).unwrap());
//            }
//        }
//    }
}

fn clear_and_write_bytes(mut buf: &[u8]) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("text.txt")
        .unwrap();
    file.set_len(0);
    file.write_all(buf).unwrap();
}


