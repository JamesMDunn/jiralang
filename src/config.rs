use ini::Ini;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub site: String,
    pub username: String,
    pub password: String,
}

pub fn create_config(site: String, username: String, password: String) -> std::io::Result<()> {
    let file_path = get_config_path();
    println!("file path is {:?}", file_path);
    let mut conf = Ini::new();
    conf.with_section(Some("Config"))
        .set("site", site)
        .set("username", username)
        .set("password", password);
    conf.write_to_file(file_path)?;
    Ok(())
}

pub fn get_config_path() -> PathBuf {
    let mut home_path = dirs::home_dir().expect("Expected a home path");
    home_path.push(".jiralang");
    home_path
}

pub fn read_config() -> Result<Config, ini::Error> {
    let file_path = get_config_path();
    let conf = Ini::load_from_file(file_path)?;
    let config_section = conf
        .section(Some("Config"))
        .expect("Expected to have config section");
    let site = config_section
        .get("site")
        .expect("expected to have property site")
        .to_owned();
    let username = config_section
        .get("username")
        .expect("expected to have property username")
        .to_owned();
    let password = config_section
        .get("password")
        .expect("expected to have property password")
        .to_owned();
    Ok(Config {
        site,
        username,
        password,
    })
}
