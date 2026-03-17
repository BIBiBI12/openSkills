use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use dialoguer::Password;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

mod crypto;
mod config;
mod env_files;

use config::Config;
use env_files::EnvManager;

const NEXUS_DIR: &str = ".env-nexus";

#[derive(Parser)]
#[command(name = "dnx")]
#[command(about = "The modern .env file manager", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize dotenv-nexus in your project
    Init,
    /// Add an existing .env file
    Add {
        /// Path to .env file
        file: PathBuf,
        /// Environment name (e.g., development, staging, production)
        #[arg(short, long)]
        env: String,
    },
    /// Encrypt all environment files
    Encrypt,
    /// Decrypt all environment files
    Decrypt,
    /// List all environments
    List,
    /// Switch to a specific environment
    Switch {
        /// Environment name
        env: String,
    },
    /// Show differences between environments
    Diff {
        /// First environment
        env1: Option<String>,
        /// Second environment
        env2: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Add { file, env } => cmd_add(file, &env),
        Commands::Encrypt => cmd_encrypt(),
        Commands::Decrypt => cmd_decrypt(),
        Commands::List => cmd_list(),
        Commands::Switch { env } => cmd_switch(&env),
        Commands::Diff { env1, env2 } => cmd_diff(env1, env2),
    }
}

fn cmd_init() -> Result<()> {
    let nexus_dir = Path::new(NEXUS_DIR);

    if nexus_dir.exists() {
        println!(
            "{}",
            "dotenv-nexus is already initialized in this directory".yellow()
        );
        return Ok(());
    }

    fs::create_dir(nexus_dir).context("Failed to create .env-nexus directory")?;

    let config = Config::new();
    config.save(&nexus_dir.join("config.json"))?;

    println!("✅ {}", "Initialized dotenv-nexus".green());
    println!("Run {} to add your first environment", "dnx add .env --env development".cyan());

    Ok(())
}

fn cmd_add(file: PathBuf, env_name: &str) -> Result<()> {
    let nexus_dir = Path::new(NEXUS_DIR);

    if !nexus_dir.exists() {
        return Err(anyhow::anyhow!(
            "dotenv-nexus not initialized. Run 'dnx init' first"
        ));
    }

    if !file.exists() {
        return Err(anyhow::anyhow!("File {} not found", file.display()));
    }

    let mut config = Config::load(&nexus_dir.join("config.json"))?;
    let env_manager = EnvManager::new(nexus_dir);

    let content = fs::read_to_string(&file)?;
    env_manager.add_environment(env_name, &content)?;

    config.add_environment(env_name);
    config.save(&nexus_dir.join("config.json"))?;

    // Generate .env.example
    env_manager.generate_example(&file)?;

    println!("✅ {} {}", "Added environment:".green(), env_name.cyan());
    println!("📝 Generated .env.example template");
    println!("🔒 Run {} to encrypt and commit to git", "dnx encrypt".cyan());

    Ok(())
}

fn cmd_encrypt() -> Result<()> {
    let nexus_dir = Path::new(NEXUS_DIR);

    if !nexus_dir.exists() {
        return Err(anyhow::anyhow!(
            "dotenv-nexus not initialized. Run 'dnx init' first"
        ));
    }

    let config = Config::load(&nexus_dir.join("config.json"))?;
    let env_manager = EnvManager::new(nexus_dir);

    println!("{}", "Enter encryption password:".yellow());
    let password = Password::new().interact()?;

    for env_name in &config.environments {
        if env_manager.has_environment(env_name) {
            env_manager.encrypt_environment(env_name, &password)?;
            println!("✅ Encrypted {}", env_name.cyan());
        }
    }

    println!("🎉 {}", "All environments encrypted!".green());
    println!("✨ Safe to commit to git now");

    Ok(())
}

fn cmd_decrypt() -> Result<()> {
    let nexus_dir = Path::new(NEXUS_DIR);

    if !nexus_dir.exists() {
        return Err(anyhow::anyhow!(
            "dotenv-nexus not initialized. Run 'dnx init' first"
        ));
    }

    let config = Config::load(&nexus_dir.join("config.json"))?;
    let env_manager = EnvManager::new(nexus_dir);

    println!("{}", "Enter decryption password:".yellow());
    let password = Password::new().interact()?;

    for env_name in &config.environments {
        if env_manager.has_encrypted(env_name) {
            env_manager.decrypt_environment(env_name, &password)?;
            println!("✅ Decrypted {}", env_name.cyan());
        }
    }

    println!("🎉 {}", "All environments decrypted!".green());

    Ok(())
}

fn cmd_list() -> Result<()> {
    let nexus_dir = Path::new(NEXUS_DIR);

    if !nexus_dir.exists() {
        return Err(anyhow::anyhow!(
            "dotenv-nexus not initialized. Run 'dnx init' first"
        ));
    }

    let config = Config::load(&nexus_dir.join("config.json"))?;

    println!("{}", "Configured environments:".bold());
    for env in &config.environments {
        let encrypted = Path::new(NEXUS_DIR)
            .join(format!("{}.enc", env))
            .exists();
        let status = if encrypted { "🔒" } else { "📝" };
        println!("  {} {}", status, env.cyan());
    }

    Ok(())
}

fn cmd_switch(env_name: &str) -> Result<()> {
    let nexus_dir = Path::new(NEXUS_DIR);

    if !nexus_dir.exists() {
        return Err(anyhow::anyhow!(
            "dotenv-nexus not initialized. Run 'dnx init' first"
        ));
    }

    let config = Config::load(&nexus_dir.join("config.json"))?;
    let env_manager = EnvManager::new(nexus_dir);

    if !config.environments.contains(&env_name.to_string()) {
        return Err(anyhow::anyhow!("Environment {} not found", env_name));
    }

    let content = env_manager.get_environment(env_name)?;
    fs::write(".env", content)?;

    println!("✅ Switched to environment {}", env_name.cyan());
    println!("📝 {} file updated", ".env".cyan());

    Ok(())
}

fn cmd_diff(env1: Option<String>, env2: Option<String>) -> Result<()> {
    let nexus_dir = Path::new(NEXUS_DIR);

    if !nexus_dir.exists() {
        return Err(anyhow::anyhow!(
            "dotenv-nexus not initialized. Run 'dnx init' first"
        ));
    }

    let config = Config::load(&nexus_dir.join("config.json"))?;
    let env_manager = EnvManager::new(nexus_dir);

    let env1_name = env1.unwrap_or_else(|| config.environments.get(0).unwrap().clone());
    let env2_name = env2.unwrap_or_else(|| {
        config
            .environments
            .get(1)
            .unwrap_or(&config.environments[0])
            .clone()
    });

    let env1_content = env_manager.get_environment(&env1_name)?;
    let env2_content = env_manager.get_environment(&env2_name)?;

    let vars1 = parse_env_vars(&env1_content);
    let vars2 = parse_env_vars(&env2_content);

    println!("{} vs {}:", env1_name.cyan(), env2_name.cyan());

    // Find differences
    let all_keys: Vec<_> = vars1
        .keys()
        .chain(vars2.keys())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    for key in all_keys {
        let val1 = vars1.get(key);
        let val2 = vars2.get(key);

        match (val1, val2) {
            (Some(v1), Some(v2)) if v1 != v2 => {
                println!("  {}:", key.yellow());
                println!("    - {}", v1);
                println!("    + {}", v2);
            }
            (Some(v1), None) => {
                println!("  {}:", key.red());
                println!("    - {}", v1);
            }
            (None, Some(v2)) => {
                println!("  {}:", key.green());
                println!("    + {}", v2);
            }
            _ => {}
        }
    }

    Ok(())
}

fn parse_env_vars(content: &str) -> HashMap<String, String> {
    let mut vars = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            vars.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    vars
}
