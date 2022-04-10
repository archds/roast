extern crate ini;

use ini::Ini;

use crate::Size;

pub struct Settings {
    pub size_limit: Size,
}

const INI_PATH: &str = "settings.ini";
const DEFAULT_SIZE_LIMIT: u64 = 1000 * 1000 * 1000;

pub fn load_settings() -> Settings {
    let settings = match Ini::load_from_file(INI_PATH) {
        Ok(ini) => Settings {
            size_limit: Size {
                bytes: ini
                    .get_from(Some("Main"), "size_limit")
                    .expect("Failed to load size_limit property!")
                    .parse::<u64>()
                    .expect("Failed to parse size_limit property as integer!"),
            },
        },
        Err(_) => {
            let mut conf = Ini::new();

            conf.with_section(Some("Main"))
                .set("size_limit", DEFAULT_SIZE_LIMIT.to_string().as_str());

            conf.write_to_file(INI_PATH)
                .expect("Failed to write config file!");

            Settings {
                size_limit: Size {
                    bytes: DEFAULT_SIZE_LIMIT,
                },
            }
        }
    };

    return settings;
}
