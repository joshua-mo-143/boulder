use boulder_server::auth::AuthBody;

use inquire::Text;
use reqwest::StatusCode;

use std::path::PathBuf;

use crate::errors::CliError;

use crate::args::{Cli, Commands, SecretsCommands, UserCommands, WebsiteCommands};

use boulder_core::secrets::KeyFile;
use crate::config::AppConfig;

pub fn parse_cli(cli: Cli, cfg: AppConfig) -> Result<(), CliError> {
    match cli.command {
        Commands::Secrets { cmd } => match cmd {
            SecretsCommands::Get { key } => {
                let Some(jwt) = cfg.clone().jwt_key() else {
                    panic!("You need to log in before you can do that!"); 
                };

                let website = match cfg.website() {
                    Some(res) => format!("{res}/secrets/get"),
                    None => panic!("You didn't set a URL for a Boulder instance to log into!"),
                };

                let ctx = reqwest::blocking::Client::new();

                let res = ctx
                    .post(website)
                    .header("Content-Type", "application/json")
                    .header("Authorization", jwt)
                    .json(&serde_json::json!({"key":key}))
                    .send()?;

                let body = res.text()?;

                println!("{body}");
            }

            SecretsCommands::Set { key, value } => {
                let Some(jwt) = cfg.clone().jwt_key() else {
                    panic!("You need to log in before you can do that!"); 
                };

                let website = match cfg.website() {
                    Some(res) => format!("{res}/secrets/set"),
                    None => panic!("You didn't set a URL for a Boulder instance to log into!"),
                };

                let ctx = reqwest::blocking::Client::new();

                let res = ctx
                    .post(website)
                    .header("Content-Type", "application/json")
                    .header("Authorization", jwt)
                    .json(&serde_json::json!({"key":key,"value":value}))
                    .send()?;

                match res.status() {
                    StatusCode::CREATED => println!("Key successfully set."),
                    _ => {
                        println!("Bad credentials: {}", res.status())
                    }
                }
            }
            SecretsCommands::List => {
                let Some(jwt) = cfg.clone().jwt_key() else {
                    panic!("You need to log in before you can do that!"); 
                };

                let website = match cfg.website() {
                    Some(res) => format!("{res}/secrets"),
                    None => panic!("You didn't set a URL for a Boulder instance to log into!"),
                };

                let ctx = reqwest::blocking::Client::new();

                let res = ctx
                    .post(website)
                    .header("Authorization", jwt)
                    .send()?;

                let body = res.json::<Vec<String>>()?;

                println!("{body:?}");

            }
            SecretsCommands::Rm { key } => {
                let Some(jwt) = cfg.clone().jwt_key() else {
                    panic!("You need to log in before you can do that!"); 
                };

                let website = match cfg.website() {
                    Some(res) => format!("{res}/secrets"),
                    None => panic!("You didn't set a URL for a Boulder instance to log into!"),
                };

                let ctx = reqwest::blocking::Client::new();

                let res = ctx
                    .delete(website)
                    .header("Authorization", jwt)
                    .json(&serde_json::json!({"key":key}))
                    .send()?;

                match res.status() {
                    StatusCode::OK => println!("Key successfully deleted."),
                    _ => println!("Error while deleting key: {}", res.text().unwrap()) 
                }

            }
        },
        Commands::Keygen(args) => {
        let key = KeyFile::new();
        let encoded = bincode::serialize(&key).unwrap();

        let mut path = match args.output {
            Some(res) => res,
            None => PathBuf::from("./boulder.bin") 
        };

        if path.as_path().is_dir() {
            path.push("boulder.bin");
        }

        std::fs::write(&path, encoded)?;

        println!("Your root key: {}", key.unseal_key());
        println!("Be sure to keep this key somewhere safe - you won't be able to get it back!");
        println!("---");
        }

        Commands::Users { cmd } => match cmd {
            UserCommands::Create => {
                let website = match cfg.website() {
                    Some(res) => format!("{res}/users/create"),
                    None => panic!("You didn't set a URL for a Boulder instance to log into!"),
                };

                let key = Text::new("Please enter your root key:").prompt()?; 

                let ctx = reqwest::blocking::Client::new();

                let res = ctx
                    .post(website)
                    .header("Content-Type", "application/json")
                    .header("x-boulder-key", key)
                    .json(&serde_json::json!({"name":"josh"}))
                    .send()?;

                let body = res.text()?;

                println!("{body}");
            }
        },
        Commands::Website { cmd } => match cmd {
            WebsiteCommands::Get => match cfg.website() {
                Some(res) => println!("{res}"),
                None => println!("No website has been set!"),
            },
            WebsiteCommands::Set { value } => {
                cfg.set_website(&value)?;
            }
        },

        Commands::Login { api_key } => {
            let ctx = reqwest::blocking::Client::new();

            let website = match cfg.to_owned().website() {
                Some(res) => format!("{res}/login"),
                None => panic!("You didn't set a URL for a Boulder instance to log into!"),
            };

            let res = ctx
                .post(website)
                .header("Content-Type", "application/json")
                .json(&serde_json::json!({"password": api_key }))
                .send()?;

            let res = res.json::<AuthBody>()?;

            let token = format!("{} {}", res.token_type, res.access_token);
            cfg.set_token(&token)?;

            println!("You've logged in successfully!");
        }

        Commands::Unseal { boulder_key } => {
            let ctx = reqwest::blocking::Client::new();

            let website = match cfg.to_owned().website() {
                Some(res) => format!("{res}/unseal"),
                None => panic!("You didn't set a URL for a Boulder instance to log into!"),
            };

            let res = ctx
                .post(website)
                .header("Content-Type", "application/json")
                .header("x-boulder-key", boulder_key)
                .send()?;

            match res.status() {
                StatusCode::OK => println!("The database has been unsealed and is ready to use!"),
                _ => {
                    println!("Bad credentials.")
                }
            }
        }
    }

Ok(())

}
