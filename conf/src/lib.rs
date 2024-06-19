use std::{
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Network {
    pub port: Option<u32>,
    pub host: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Global {
    pub enable_blur: Option<bool>,
    pub lang: Option<String>,
    pub theme: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub global: Option<Global>,
    pub network: Option<Network>,
}

impl Config {
    fn default() -> (Config, String) {
        let data = include_str!("./default.toml");
        let data = data.replace(
            "{{LANG}}",
            &sys_locale::get_locale().unwrap_or("en-GB".to_string()),
        );

        (toml::from_str(&data).unwrap(), data)
    }

    pub fn dump(path: &PathBuf, new_config: Config) {
        let mut f = std::fs::File::create(path).unwrap();
        let _ = f.write_all(toml::to_string(&new_config).unwrap().as_bytes());
    }

    pub fn get(path: &PathBuf) -> Config {
        let mut buf = String::new();
        if path.exists() {
            let mut f = std::fs::File::open(path).unwrap();
            let _ = f.read_to_string(&mut buf);

            toml::from_str(&buf).unwrap()
        } else {
            let (conf, conf_s) = Config::default();
            let mut f = std::fs::File::create(path).unwrap();
            let _ = f.write_all(conf_s.as_bytes());
            conf
        }
    }

    /// Function to update the global section
    pub fn update_global(&mut self, new_global: Global) {
        self.global = Some(new_global);
    }

    /// Function to update the network section
    pub fn update_network(&mut self, new_network: Network) {
        self.network = Some(new_network);
    }

    /// Function to update a specific field in the global section
    pub fn update_global_field<F>(&mut self, update_fn: F)
    where
        F: FnOnce(&mut Global),
    {
        if let Some(ref mut global) = self.global {
            update_fn(global);
        } else {
            let mut new_global = Global {
                enable_blur: None,
                lang: None,
                theme: None,
            };
            update_fn(&mut new_global);
            self.global = Some(new_global);
        }
    }

    /// Function to update a specific field in the network section
    pub fn update_network_field<F>(&mut self, update_fn: F)
    where
        F: FnOnce(&mut Network),
    {
        if let Some(ref mut network) = self.network {
            update_fn(network);
        } else {
            let mut new_network = Network {
                port: None,
                host: None,
            };
            update_fn(&mut new_network);
            self.network = Some(new_network);
        }
    }

    /// Function to load, update, and save the configuration
    pub fn load_update_save(path: &PathBuf, update_fn: impl FnOnce(&mut Config)) {
        let mut config = Config::get(path);
        update_fn(&mut config);
        Config::dump(path, config);
    }
}
