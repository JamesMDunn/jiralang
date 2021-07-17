use clap::{App, Arg, SubCommand};
use prettytable::{Cell, Row, Table};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{self, Write};
use tokio;
mod config;

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

#[derive(Serialize, Deserialize, Debug)]
struct ProjectAvatar {
    #[serde(alias = "48x48")]
    forty_eight: String,
    #[serde(alias = "24x24")]
    twenty_four: String,
    #[serde(alias = "16x16")]
    sixteen: String,
    #[serde(alias = "32x32")]
    thirty_two: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectCategory {
    #[serde(rename = "self")]
    self_url: String,
    id: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectInsight {
    total_issue_count: u32,
    last_issue_update_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ProjectValues {
    expand: String,
    #[serde(rename = "self")]
    self_url: String,
    id: String,
    key: String,
    name: String,
    simplified: bool,
    style: String,
    is_private: bool,
    entity_id: String,
    uuid: String,
    avatar_urls: ProjectAvatar,
    project_category: Option<ProjectCategory>,
    insight: Option<ProjectInsight>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Project {
    #[serde(rename = "self")]
    self_url: String,
    next_page: Option<String>,
    max_results: u32,
    start_at: u32,
    total: u32,
    is_last: bool,
    values: Vec<ProjectValues>,
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
    config::create_config(site, username, password).expect("Failed to create config");
    let conf = config::read_config().expect("tried to read config");
    println!("{:?} this is your config", conf);
    Ok(())
}

async fn get_jira_board() -> Result<(), reqwest::Error> {
    let res = get_client_request("/rest/agile/1.0/board").await?;
    let deserialized = serde_json::from_str::<JiraBoard>(&res).expect("failed to deserialize json");

    println!("{:?}", deserialized);
    Ok(())
}

async fn get_client_request(endpoint: &str) -> Result<String, reqwest::Error> {
    let config = config::read_config().expect("Expected to read config");
    let client = reqwest::Client::new();
    let res = client
        .get(config.site + &endpoint)
        .basic_auth(config.username, Some(config.password))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

async fn get_projects() -> Result<(), reqwest::Error> {
    let res = get_client_request("/rest/api/3/project/search").await?;
    let projects = serde_json::from_str::<Project>(&res).expect("Failed to deserialize data");

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Name"),
        Cell::new("View"),
    ]));
    for proj in projects.values {
        table.add_row(Row::new(vec![
            Cell::new(&proj.id),
            Cell::new(&proj.name),
            Cell::new(&proj.self_url),
        ]));
    }
    table.printstd();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = App::new("Jiralang program")
        .version("0.01")
        .author("James Dunn")
        .subcommand(SubCommand::with_name("config").about("setup config to login to jira"))
        .subcommand(SubCommand::with_name("board").about("Retrieve jira board"))
        .subcommand(
            SubCommand::with_name("projects")
                .about("Jira projects")
                .arg(Arg::with_name("get")),
        )
        .get_matches();

    match matches.subcommand() {
        ("board", Some(_)) => get_jira_board().await?,
        ("config", Some(_)) => config().await?,
        ("projects", Some(cmd)) => {
            if let Some(_) = cmd.value_of("get") {
                get_projects().await?;
            }
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
