#![windows_subsystem = "windows"]
mod settings;

extern crate fs_extra;
extern crate fstrings;
extern crate home;
extern crate humansize;
extern crate winrt_notification;

use core::panic;

use fs_extra::dir::get_size;
use home::home_dir;
use humansize::{file_size_opts as fs_options, FileSize};
use settings::load_settings;
use winrt_notification::Toast;

pub struct Size {
    pub bytes: u64,
}

impl Size {
    fn humansize(&self) -> String {
        return self.bytes.file_size(fs_options::DECIMAL).unwrap();
    }
}

fn notify(size: Size, dirname: &String) {
    let message =
        String::from(dirname) + " directory exceeded " + size.humansize().as_str() + " limit";

    Toast::new("Microsoft.WindowsTerminal_8wekyb3d8bbwe!App")
        .title("Roast")
        .text1(message.as_str())
        .show()
        .expect("unable to toast");
}

fn main() {
    let path = home_dir()
        .expect("Failed to locate home dir!")
        .join("Downloads");

    if !path.exists() {
        panic!("Downloads path not found in {}!", path.to_str().unwrap())
    }

    let downloads_folder_size = get_size(home_dir().unwrap().join("repos")).unwrap();
    let settings = load_settings();

    if downloads_folder_size > settings.size_limit.bytes {
        notify(
            settings.size_limit,
            &String::from(path.file_name().unwrap().to_str().unwrap()),
        )
    }
}
