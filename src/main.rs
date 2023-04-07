mod creator;
mod format;
#[cfg(test)]
mod tests;

use clap::{command, Parser, Subcommand};
use creator::{
    cpp_script::CppScriptCreator, traits::namespaced_creator::NamespacedCreator, Creator,
};
use log::{error, info};

use crate::creator::cpp_class::CppClassCreator;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<SubCmds>,
}

#[derive(Subcommand)]
enum SubCmds {
    #[command(alias = "i")]
    /// Initialize a new project
    Init {
        /// The name of the project
        #[arg()]
        name: String,
    },

    #[command(alias = "n")]
    /// Create a new file
    New {
        /// The type of new file
        #[command(subcommand)]
        file_type: FileTypeSubCmds,
    },
}

#[derive(Subcommand)]
enum FileTypeSubCmds {
    #[command(alias = "s")]
    /// Create a new script
    Script {
        /// The name of the script
        #[arg()]
        name: String,
        // The namespace of the script
        #[arg(short, long)]
        namespace: Option<String>,
    },

    #[command(alias = "c")]
    /// Create a new class
    Class {
        /// The name of the class
        #[arg()]
        name: String,
        // The namespace of the class
        #[arg(short, long)]
        namespace: Option<String>,
    },
}

fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let cli = Cli::parse();

    match cli.command {
        Some(SubCmds::Init { name }) => {
            info!("Initializing project {}", name);
        }
        Some(SubCmds::New { file_type }) => match file_type {
            FileTypeSubCmds::Script { name, namespace } => {
                let creator = CppScriptCreator::new(&name);
                let creator = match namespace {
                    Some(namespace) => creator.with_namespace(&namespace),
                    None => creator,
                };
                match creator.save() {
                    Ok(_) => (),
                    Err(e) => error!("Error creating file: {}", e),
                }
            }
            FileTypeSubCmds::Class { name, namespace } => {
                info!(
                    "Creating class {}{}",
                    match namespace {
                        Some(ref namespace) => format!("{}::", namespace),
                        None => "".to_string(),
                    },
                    name
                );
                let creator = CppClassCreator::new(&name);
                let creator = match namespace {
                    Some(ref namespace) => creator.with_namespace(&namespace),
                    None => creator,
                };
                match creator.save() {
                    Ok(_) => (),
                    Err(e) => error!("Error creating file: {}", e),
                }
            }
        },
        None => {
            error!("No command given");
        }
    }
}
