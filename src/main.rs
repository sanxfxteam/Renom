use clap::Parser;
use lmk::init_crash_reporter;
use renom::{
    cli::{
        Cli,
        Command::{RenameModule, RenamePlugin, RenameProject, RenameTarget, Wizard},
    },
    presentation::log,
    wizard::start_interactive_dialogue,
    workflows::{rename_module, rename_plugin, rename_project, rename_target},
};

fn main() {
    init_crash_reporter!();

    let cli = Cli::parse();

    // Set verbose mode if the flag is present
    log::set_verbose(cli.verbose);

    match cli.command {
        None => { /* noop, clap will handle top-level help and version */ }
        Some(command) => {
            if let Err(e) = match command {
                RenameProject(params) => rename_project(params.into_params(cli.verbose)),
                RenamePlugin(params) => rename_plugin(params.into_params(cli.verbose)),
                RenameTarget(params) => rename_target(params.into_params(cli.verbose)),
                RenameModule(params) => rename_module(params.into_params(cli.verbose)),
                Wizard => {
                    start_interactive_dialogue();
                    Ok(())
                }
            } {
                log::error(e);
            }
        }
    };
}
