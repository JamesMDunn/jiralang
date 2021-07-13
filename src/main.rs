use clap::{Arg, App, SubCommand};
use std::io::{self, Write};
use std::collections::HashMap;
use reqwest;
use tokio;
use base64::{encode};

// struct User { 
//    email: String,
//  password: String,
//}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = App::new("Jiralang program")
        .version("0.01")
        .author("James Dunn")
        .subcommand(
            SubCommand::with_name("login")
                .about("Login to jira"))
        .get_matches();

    match matches.subcommand() {
        ("login", Some(sub_m)) => {

          let mut site = String::new();
          print!("Site: ");
          io::stdout().flush().expect("Failed to flush");
          io::stdin().read_line(&mut site).expect("error unable to read user");
          trim_newline(&mut site);

          let mut username = String::new();
          print!("username: ");
          io::stdout().flush().expect("Failed to flush");
          io::stdin().read_line(&mut username).expect("error unable to read user");
          trim_newline(&mut username);
          println!("this is user {:?}", username);

          let mut password = String::new();
          print!("password: ");
          io::stdout().flush().expect("Failed to flush?");
          io::stdin().read_line(&mut password).expect("error unable to read user");
          trim_newline(&mut password);
          let merge = [username, ":".to_owned(), password].join("");
          println!("merge {:?}", merge);
          let encoded = encode(merge);
          let client = reqwest::Client::new();
          let res = client.get(site + "/rest/agile/1.0/board")
            .header(reqwest::header::AUTHORIZATION, "Basic ".to_owned() + &encoded)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await?;
          //let json = res.json().await?; 
          println!("{:?}",res.text().await?);
          println!("{:?}", encoded);

        },
        (command, _) => unreachable!("invalid subcommand: {}", command),
    }
    Ok(())
}


fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
    }
}
