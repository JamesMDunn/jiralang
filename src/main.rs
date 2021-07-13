use base64::encode;
use clap::{App, Arg, SubCommand};
use reqwest;
use std::collections::HashMap;
use std::io::{self, Write};
use tokio;

// struct User {
//    email: String,
//  password: String,
//
//}

fn get_user_input() -> String {
    let mut input = String::new(); 
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("error unable to read user");
    trim_newline(&mut input);
    input

}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = App::new("Jiralang program")
        .version("0.01")
        .author("James Dunn")
        .subcommand(SubCommand::with_name("login").about("Login to jira"))
        .get_matches();

    match matches.subcommand() {
        ("login", Some(sub_m)) => {
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
                .await?;
            //let json = res.json().await?;
            println!("{:?}", res.text().await?);
            println!("{:?}", encoded);
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
