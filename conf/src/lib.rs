use std::{
    io::{Read, Write},
    path::PathBuf,
};

#[macro_export]
macro_rules! update_conf {
    ($config:expr, $category:ident, $field:ident, $value:expr) => {{
        if let Some(ref mut cat) = $config.$category {
            cat.$field = $value;
        } else {
            let mut cat = <$crate::Config as Default>::default().$category.unwrap();
            cat.$field = $value;
            $config.$category = Some(cat);
        }
    }};
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Network {
    pub port: Option<u32>,
    pub host: Option<String>,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            port: Some(7700),
            host: Some("localhost".to_string()),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Global {
    pub enable_blur: Option<bool>,
    pub lang: Option<String>,
    pub theme: Option<String>,
}

impl Default for Global {
    fn default() -> Self {
        let l = sys_locale::get_locale().unwrap_or("en-GB".to_string());
        Self {
            enable_blur: Some(false),
            lang: Some(l),
            theme: Some("auto".to_string()),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub global: Option<Global>,
    pub network: Option<Network>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            global: Some(Global::default()),
            network: Some(Network::default()),
        }
    }
}

impl Config {
    pub fn dump(path: &PathBuf, new_config: Config) {
        let mut f = std::fs::File::create(path).unwrap();
        let _ = f.write_all(toml::to_string(&new_config).unwrap().as_bytes());
    }

    pub fn get(path: &PathBuf) -> Config {
        let mut buf = String::new();
        if path.exists() {
            let mut f = std::fs::File::open(path).unwrap();
            let _ = f.read_to_string(&mut buf);

            match toml::from_str::<Config>(&buf) {
                Ok(parsed_conf) => parsed_conf,
                Err(_) => Config::default(),
            }
        } else {
            let conf = Config::default();
            Config::dump(path, conf.clone());
            conf
        }
    }
}
