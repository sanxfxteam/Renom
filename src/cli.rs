use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::workflows::{rename_module, rename_plugin, rename_project, rename_target};

#[derive(Parser)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
    /// Enable verbose logging to see detailed operation information
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,
}

#[derive(PartialEq, Debug, Subcommand)]
pub enum Command {
    /// Rename an Unreal Engine project
    RenameProject(RenameProject),
    /// Rename an Unreal Engine project plugin
    RenamePlugin(RenamePlugin),
    /// Rename an Unreal Engine project target
    RenameTarget(RenameTarget),
    /// Rename an Unreal Engine project module
    RenameModule(RenameModule),
    /// Start an interactive session
    Wizard,
}

#[derive(PartialEq, Debug, Parser)]
pub struct RenameProject {
    /// Path to the project to rename
    #[arg(long)]
    project: PathBuf,
    /// New name for the project
    #[arg(long)]
    new_name: String,
}

impl RenameProject {
    pub fn into_params(self, verbose: bool) -> rename_project::Params {
        rename_project::Params {
            project_root: self.project,
            new_name: self.new_name,
            verbose,
        }
    }
}

#[derive(PartialEq, Debug, Parser)]
pub struct RenamePlugin {
    /// Path to the project that the plugin is part of
    #[arg(long)]
    project: PathBuf,
    /// Plugin in the project to rename
    #[arg(long)]
    plugin: String,
    /// New name for the plugin
    #[arg(long)]
    new_name: String,
}

impl RenamePlugin {
    pub fn into_params(self, verbose: bool) -> rename_plugin::Params {
        rename_plugin::Params {
            project_root: self.project,
            plugin: self.plugin,
            new_name: self.new_name,
            verbose,
        }
    }
}

#[derive(PartialEq, Debug, Parser)]
pub struct RenameTarget {
    /// Path to the project that the target is part of
    #[arg(long)]
    project: PathBuf,
    /// Target in the project to rename
    #[arg(long)]
    target: String,
    /// New name for the target
    #[arg(long)]
    new_name: String,
}

impl RenameTarget {
    pub fn into_params(self, verbose: bool) -> rename_target::Params {
        rename_target::Params {
            project_root: self.project,
            target: self.target,
            new_name: self.new_name,
            verbose,
        }
    }
}

#[derive(PartialEq, Debug, Parser)]
pub struct RenameModule {
    /// Path to the project that the module is part of
    #[arg(long)]
    project: PathBuf,
    /// Module in the project to rename
    #[arg(long)]
    module: String,
    /// New name for the module
    #[arg(long)]
    new_name: String,
}

impl RenameModule {
    pub fn into_params(self, verbose: bool) -> rename_module::Params {
        rename_module::Params {
            project_root: self.project,
            module: self.module,
            new_name: self.new_name,
            verbose,
        }
    }
}
