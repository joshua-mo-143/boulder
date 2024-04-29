use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Commands related to secrets management.
    Secrets {
        #[command(subcommand)]
        cmd: SecretsCommands,
    },
    /// Commands related to user management. Note that your root key is required for this.
    Users {
        #[command(subcommand)]
        cmd: UserCommands,
    },
    /// Commands related to setting/getting the URL for your Chamber instance.
    Website {
        #[command(subcommand)]
        cmd: WebsiteCommands,
    },
    /// Log in to your Chamber instance.
    Login(LoginArgs),
    /// Commands related to generating keys for your Chamber instance.
    Keygen(KeygenArgs),
    /// Unseal your Chamber instance.
    Unseal {
        chamber_key: String,
    },
    Upload(UploadArgs),
    Ssh,
}

#[derive(Parser, Clone)]
pub struct LoginArgs {
    #[arg(long, short = 'u')]
    pub username: Option<String>,
    #[arg(long, short = 'p')]
    pub password: Option<String>,
}

#[derive(Parser, Clone)]
pub struct UserArgs {
    #[arg(long, short = 'u')]
    pub username: Option<String>,
}
#[derive(Parser, Clone)]
pub struct ListArgs {
    #[arg(long, short = 't')]
    pub tags: Option<String>,
}

#[derive(Parser, Clone)]
pub struct KeyArgs {
    #[arg(long, short = 'k')]
    pub key: Option<String>,
}
#[derive(Parser, Clone)]
pub struct KeygenArgs {
    /// Provide a root key. Randomly generated by default.
    #[arg(long, short = 'k')]
    pub key: Option<String>,
    /// Provide a known destination for your file output.
    /// Note that you need a .bin file extension.
    #[arg(long, short = 'o')]
    pub output: Option<PathBuf>,
}

#[derive(Parser, Clone)]
pub struct UpdateUserArgs {
    pub username: String,
    pub access_level: Option<i32>,
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub roles: Option<Vec<String>>,
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// Create a new user
    Create(LoginArgs),
    /// Create a new user
    Update(UpdateUserArgs),
    /// Create a new user
    Delete(UserArgs),
}

#[derive(Subcommand)]
pub enum SecretsCommands {
    /// Decrypt and view a secret stored in your Boulder instance
    Get(KeyArgs),
    /// Create a new secret
    Set { key: String, value: String },
    /// Update a secret
    Update {
        key: String,
        #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        tags: Vec<String>,
    },
    /// List the names of all secrets currently stored (that you have access to)
    List(ListArgs),
    /// List decrypted secrets of all secrets by tag
    ListByTag(ListByTagArgs),
    /// Delete a secret
    Rm(KeyArgs),
}

#[derive(Parser, Clone)]
pub struct ListByTagArgs {
    pub key: String
}

#[derive(Subcommand)]
pub enum WebsiteCommands {
    /// Get the current URL
    Get,
    /// Set the current URL
    Set(SetArgs),
}

#[derive(Parser, Clone)]
pub struct SetArgs {
    #[arg(long, short = 'v')]
    pub value: Option<String>,
}

#[derive(Parser, Clone)]
pub struct UploadArgs {
    #[arg(long, short)]
    pub key: Option<String>,
}
