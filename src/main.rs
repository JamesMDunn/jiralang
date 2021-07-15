use clap::{App, Arg, SubCommand};
use dirs;
use ini::Ini;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tokio;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JiraBoardLocation {
    project_id: u64,
    display_name: String,
    project_name: String,
    project_key: String,
    project_type_key: String,
    #[serde(alias = "avatarURI")]
    avatar_uri: String,
    name: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JiraBoardValues {
    id: u32,
    #[serde(rename = "self")]
    self_type: String,
    name: String,
    #[serde(alias = "type")]
    type_name: String,
    location: JiraBoardLocation,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JiraBoard {
    max_results: u32,
    start_at: u32,
    total: u32,
    is_last: bool,
    values: Vec<JiraBoardValues>,
}

struct User {
    site: String,
    username: String,
    password: String,
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("error unable to read input");
    trim_newline(&mut input);
    input
}

async fn config() -> Result<(), reqwest::Error> {
    print!("Site: ");
    let site = get_user_input();

    print!("username: ");
    let username = get_user_input();

    print!("password: ");
    let password = get_user_input();
    create_config(site, username, password).expect("Failed to create config");

    Ok(())
}

async fn get_jira_board() -> Result<(), reqwest::Error> {
    //let res = get_client_request(("/rest/agile/1.0/board", username, password).await?;
    //let deserialize_jiraboard =
    //serde_json::from_str::<JiraBoard>(&res).expect("failed to deserialize json");

    //println!("{:?}", deserialize_jiraboard);
    Ok(())
}

async fn get_client_request(
    endpoint: String,
    username: String,
    password: String,
) -> Result<reqwest::Client, reqwest::Error> {
    let client = reqwest::Client::new();
    let site = String::new();
    client
        .get(site + &endpoint)
        .basic_auth(username, Some(password))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(client)
}

fn create_config(site: String, username: String, password: String) -> std::io::Result<()> {
    let mut file_path = get_config_path();
    println!("file path is {:?}", file_path);
    let mut conf = Ini::new();
    conf.with_section(
        Some("Config"))
            .set("site", site)
            .set("username", username)
            .set("password", password);
    conf.write_to_file(file_path)?;
    Ok(())
}

fn get_config_path() -> PathBuf {
    let mut home_path = dirs::home_dir().expect("Expected a home path");
    home_path.push(".jiralang");
    home_path
}

fn read_config() -> std::io::Result<()> {
    let mut home_path = get_config_path();
    let mut file = File::open(home_path.as_path())?;
    println!("{:?}", file);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = App::new("Jiralang program")
        .version("0.01")
        .author("James Dunn")
        .subcommand(SubCommand::with_name("config").about("setup config to login to jira"))
        .get_matches();

    match matches.subcommand() {
        ("config", Some(_)) => config().await?,
        (command, _) => unreachable!("invalid subcommand: {}", command),
    }
    Ok(())
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
    }
}
