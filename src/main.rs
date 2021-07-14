use base64::encode;
use clap::{App, Arg, SubCommand};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::io::{self, Write};
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

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("error unable to read input");
    trim_newline(&mut input);
    input
}

async fn login() -> Result<(), reqwest::Error> {
    print!("Site: ");
    let site = get_user_input();

    print!("username: ");
    let username = get_user_input();

    print!("password: ");
    let password = get_user_input();

    let merge = [username, ":".to_owned(), password].join("");
    println!("merge {:?}", merge);
    let encoded = encode(merge);
    let client = reqwest::Client::new();
    let res = client
        .get(site + "/rest/agile/1.0/board")
        .header(
            reqwest::header::AUTHORIZATION,
            "Basic ".to_owned() + &encoded,
        )
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?
        .text()
        .await?;
    let deserialize_jiraboard =
        serde_json::from_str::<JiraBoard>(&res).expect("failed to deserialize json");

    println!("{:?}", deserialize_jiraboard);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = App::new("Jiralang program")
        .version("0.01")
        .author("James Dunn")
        .subcommand(SubCommand::with_name("login").about("Login to jira"))
        .get_matches();

    match matches.subcommand() {
        ("login", Some(_)) => {
            login().await?;
        }
        (command, _) => unreachable!("invalid subcommand: {}", command),
    }
    Ok(())
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
    }
}
